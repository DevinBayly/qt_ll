use rand::prelude::*;
use std::cell::RefCell;
use std::env::args;
use std::io::prelude::*;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use std::fs::write;

struct Extent {
    min: f32,
    max: f32,
}

impl Extent {
    fn new() -> Self {
        Extent { min: 0.0, max: 0.0 }
    }
    fn comp(&mut self, other: f32) {
        if other < self.min {
            self.min = other;
        } else if other > self.max {
            self.max = other;
        }
    }
}

#[derive(Debug)]
enum El<T> {
    None,
    Some(Rc<RefCell<T>>),
}

impl<T> El<T> {
    fn new_part(data: T) -> Rc<RefCell<T>> {
        Rc::new(RefCell::new(data))
    }
}
#[derive(Debug)]
struct Tree {
    val: f32,
}
#[derive(Debug, Clone, PartialEq)]
struct PT {
    x: f32,
    y: f32,
}
impl PT {
    fn new(x: f32, y: f32) -> Self {
        PT { x, y }
    }
}
// bounding box
// width,height,center,t,b,l,r values
// methods, contains, new
#[derive(Debug)]
struct BB {
    w: f32,
    h: f32,
    c: PT,
    t: f32,
    b: f32,
    l: f32,
    r: f32,
}

impl BB {
    fn new(c: PT, w: f32, h: f32) -> BB {
        BB {
            t: c.y + h / 2.0,
            b: c.y - h / 2.0,
            l: c.x - w / 2.0,
            r: c.x + w / 2.0,
            w,
            h,
            c,
        }
    }
    fn contains(&self, o: &PT) -> bool {
        self.t >= o.y && self.b <= o.y && self.l <= o.x && self.r >= o.x
    }
}

#[derive(Debug)]
struct QT {
    points: Vec<PT>,
    capacity: usize,
    subdiv: bool,
    margin: f32,
    bb: BB,
    ne: El<QT>,
    nw: El<QT>,
    se: El<QT>,
    sw: El<QT>,
}

impl QT {
    // include subdiv
    // include capacity
    // create rect group
    // support overlap margin idea
    fn new(c: PT, w: f32, h: f32, margin: f32, cap: usize) -> Self {
        QT {
            points: vec![],
            bb: BB::new(c, w, h),
            margin,
            capacity: cap,
            subdiv: false,
            ne: El::None,
            nw: El::None,
            se: El::None,
            sw: El::None,
        }
    }
    // need the add point algorithm
    fn add_point(&mut self, o: PT, leafs: &mut Vec<Rc<RefCell<QT>>>) {
        // these are the children we might add the point to
        // start with head and proceed
        ////println!("num leafs {:?}",leafs.len());
        let mut new_leafs = vec![];
        println!("leafs len{}",leafs.len());
        if !self.subdiv {
            if self.points.len() < self.capacity && self.bb.contains(&o) {
                //////println!("beginning {:?}", self.points);
                self.points.push(o);
            } else {
                self.points.push(o.clone());
                self.subdivide(&mut new_leafs);
                *leafs = new_leafs;
                //////println!("after subdiv head {:?}", self.points);
                //////println!("self is subdivided {:?}",self.subdiv);
                //////println!("self is {:#?}",self);
            }
        } else {
            // unpack via popping from the leafs, but add them to the new leafs as we go under certain circumstances
            ///println!("first leafs {:?}",leafs);
            while let Some(candidate_rc) = leafs.pop() {
            ////println!("looping leafs {:?}",leafs);
                //////println!("candidate len is {:?}",candidates.len());
                let mut candidate = candidate_rc.borrow_mut();
                //println!("candidate is {:?}",candidate);
                if candidate.bb.contains(&o) {
                    if !candidate.subdiv {
                        // if capacity isn't full and haven't subdivided
                        if candidate.points.len() < candidate.capacity {
                        //println!("candidate  {:#?}contains {:?}",candidate,o);
                            candidate.points.push(o.clone());
                            // we should keep this leaf in our listing
                            new_leafs.push(candidate_rc.clone());
                            ////println!("new leafs {:?}",new_leafs);
                        } else {
                            // this is the complex spot, call on the subdivide for our candidate and spread the existing points between the children
                            // this is the end of this path too
                            // temporarily go above cap so that we can loop over the cap +1 points when redistributing
                            //println!("candidate  {:#?} needs to subdivide {:?}",candidate,o);
                            candidate.points.push(o.clone());
                            candidate.subdivide(&mut new_leafs);
                        }
                        // else if we are at cap and haven't subdivided
                    }
                } else {
                    //println!("candidate  {:#?}doesn't contain {:?}",candidate,o);
                    new_leafs.push(candidate_rc.clone());
                }
                // slowly deplete the list of candidates while we add
            }
            ////println!("new leafs end{:?}",new_leafs);    
            *leafs = new_leafs;
        }
    }
    fn subdivide(&mut self, leafs_vec: &mut Vec<Rc<RefCell<QT>>>) {
        println!("subdividing {:?}",self.bb.c);
        // make the 4 new children to replace the None's
        // pay special attention to the calculation of BB's for each
        self.subdiv = true;
        let points = self.points.clone();
        self.points = vec![];
        // subtract w/4 and add h/4 from self.c for the new center,
        // new width is w/2 + 2*margin same for height
        let mut ne = QT::new(
            PT::new(self.bb.c.x - self.bb.w / 4.0, self.bb.c.y + self.bb.h / 4.0),
            self.bb.w / 2.0 + 2.0 * self.margin,
            self.bb.h / 2.0 + 2.0 * self.margin,
            self.margin,
            self.capacity,
        );
        let mut nw = QT::new(
            PT::new(self.bb.c.x + self.bb.w / 4.0, self.bb.c.y + self.bb.h / 4.0),
            self.bb.w / 2.0 + 2.0 * self.margin,
            self.bb.h / 2.0 + 2.0 * self.margin,
            self.margin,
            self.capacity,
        );
        let mut sw = QT::new(
            PT::new(self.bb.c.x + self.bb.w / 4.0, self.bb.c.y - self.bb.h / 4.0),
            self.bb.w / 2.0 + 2.0 * self.margin,
            self.bb.h / 2.0 + 2.0 * self.margin,
            self.margin,
            self.capacity,
        );
        let mut se = QT::new(
            PT::new(self.bb.c.x - self.bb.w / 4.0, self.bb.c.y - self.bb.h / 4.0),
            self.bb.w / 2.0 + 2.0 * self.margin,
            self.bb.h / 2.0 + 2.0 * self.margin,
            self.margin,
            self.capacity,
        );
        for pt in points {
            // use direct add on each
            // they will succeed or fail depending on their bounds
            ne.directAdd(pt.clone());
            nw.directAdd(pt.clone());
            sw.directAdd(pt.clone());
            se.directAdd(pt.clone());
        }
        // then wrap them in El::Some's and update our self
        let rc_wrap_ne = El::new_part(ne);
        let rc_wrap_se = El::new_part(se);
        let rc_wrap_nw = El::new_part(nw);
        let rc_wrap_sw = El::new_part(sw);
        leafs_vec.push(rc_wrap_ne.clone());
        leafs_vec.push(rc_wrap_se.clone());
        leafs_vec.push(rc_wrap_nw.clone());
        leafs_vec.push(rc_wrap_sw.clone());
        //println!("in subdivide leafs_vec is {:#?}",leafs_vec);
        ////println!("leaf vec in subdivide {:?}",leafs_vec);
        self.ne = El::Some(rc_wrap_ne.clone());
        self.se = El::Some(rc_wrap_se.clone());
        self.nw = El::Some(rc_wrap_nw.clone());
        self.sw = El::Some(rc_wrap_sw.clone());
    }
    fn directAdd(&mut self, o: PT) {
        if self.bb.contains(&o) {
            self.points.push(o.clone());
        }
    }
    fn return_rc(el: &El<QT>, v: &mut Vec<Rc<RefCell<QT>>>) {
        match el {
            El::Some(contents) => {
                v.push(Rc::clone(&contents));
            }
            _ => {}
        }
    }
    // the query algorithm
}
// test out whether this works !!!!
// still needs initial setup and random point creation as we loop
fn main() {
    //////println!("Hello, world!");
    let max_number = args().nth(1).unwrap().parse::<usize>().unwrap();
    let mut thread = rand::thread_rng();
    let mut data = vec![];
    for i in 0..max_number {
        data.push(PT::new(thread.gen::<f32>(), thread.gen::<f32>()));
    }
    //thread.fill_data(&mut rand_data);
    let mut x_ext = Extent::new();
    let mut z_ext = Extent::new();
    for pt in data.iter() {
        x_ext.comp(pt.x);
        z_ext.comp(pt.y);
    }
    // center should be the mid point which is (max - min)/2 + min
    let w = x_ext.max - x_ext.min;
    let h = z_ext.max - z_ext.min;
    let c = PT::new(
        (x_ext.max - x_ext.min) / 2.0 + x_ext.min,
        (z_ext.max - z_ext.min) / 2.0 + z_ext.min,
    );
    let head = El::new_part(QT::new(c, w, h, w/200.0 , 4));
    let head_ref = Rc::clone(&head);
    let mut leafs = vec![];
    for (i, pt) in data.iter().enumerate() {
        println!("point processed {:?} {:?}", pt, i);
        head_ref.borrow_mut().add_point(pt.clone(), &mut leafs);
    }
    //println!("head {:?}",head);
    // iterate over the head and output data to draw the tree



}
