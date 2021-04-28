use std::cell::RefCell;
use std::rc::Rc;
// worry about the extent objects later
#[derive(Debug)]
enum El<T> {
    None,
    Some(Rc<RefCell<Box<T>>>),
}

impl<T> El<T> {
    fn new_part(data: T) -> Rc<RefCell<Box<T>>> {
        Rc::new(RefCell::new(Box::new(data)))
    }
}
#[derive(Debug)]
struct Tree {
    val: f32,
}
#[derive(Debug, Clone)]
struct PT {
    x: f32,
    y: f32,
}
impl PT{
    fn new(x:f32,y:f32) -> Self {
        PT {
            x,
            y
        }
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
            w,
            h,
            c,
            t: c.y + h / 2.0,
            b: c.y - h / 2.0,
            l: c.x - w / 2.0,
            r: c.x + w / 2.0,
        }
    }
    fn contains(&self, o: &PT) -> bool {
        self.t >= o.y && self.b <= o.y && self.l <= o.x && self.b >= o.x
    }
}

#[derive(Debug)]
struct QT {
    points: Vec<PT>,
    capacity: usize,
    subdiv: bool,
    margin:f32,
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
    fn new(c: PT, w: f32, h: f32,margin:f32, cap: usize) -> Self {
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
    fn addPoint(&mut self, o: PT) {
        // these are the children we might add the point to
        // start with head and proceed
        if self.points.len() < self.capacity && self.bb.contains(&o) {
            self.points.push(o);
        } else {
            // descend into the structure via children
            let mut candidates = vec![];
            QT::return_rc(self.ne, &mut candidates);
            QT::return_rc(self.nw, &mut candidates);
            QT::return_rc(self.se, &mut candidates);
            QT::return_rc(self.sw, &mut candidates);
            let mut candidate_option = candidates.pop();
            while let Some(candidate) = candidate_option {
                let candidate = candidate.borrow_mut();
                if candidate.bb.contains(&o) {
                    if !candidate.subdiv {
                        // if capacity isn't full and haven't subdivided
                        if candidate.points.len() < candidate.capacity {
                            candidate.points.push(o.clone());
                        } else {
                            // this is the complex spot, call on the subdivide for our candidate and spread the existing points between the children
                            // this is the end of this path too
                            // temporarily go above cap so that we can loop over the cap +1 points when redistributing
                            candidate.points.push(o.clone());
                            candidate.subdivide();
                        }
                        // else if we are at cap and haven't subdivided
                    } else {
                        // lastly, if we've already subdivided must offer up children as add point candidates
                        QT::return_rc(candidate.ne, &mut candidates);
                        QT::return_rc(candidate.nw, &mut candidates);
                        QT::return_rc(candidate.se, &mut candidates);
                        QT::return_rc(candidate.sw, &mut candidates);
                    }
                }
                // slowly deplete the list of candidates while we add
                candidate_option = candidates.pop();
            }
        }
    }
    fn subdivide(&mut self) {
        // make the 4 new children to replace the None's
        // pay special attention to the calculation of BB's for each 
        let points = self.points.clone();
        self.points = vec![];
        // subtract w/4 and add h/4 from self.c for the new center, 
        // new width is w/2 + 2*margin same for height
        let mut ne = QT::new(
            PT::new(
                self.bb.c.x - self.bb.w/4.0,
                self.bb.c.y + self.bb.h/4.0
            ),
            self.bb.w/2.0 +2.0*self.margin,
            self.bb.h/2.0 +2.0*self.margin,
            self.margin,
            self.capacity,
        );
        let mut nw = QT::new(
            PT::new(
                self.bb.c.x + self.bb.w/4.0,
                self.bb.c.y + self.bb.h/4.0
            ),
            self.bb.w/2.0 +2.0*self.margin,
            self.bb.h/2.0 +2.0*self.margin,
            self.margin,
            self.capacity,
        );
        let mut sw = QT::new(
            PT::new(
                self.bb.c.x + self.bb.w/4.0,
                self.bb.c.y - self.bb.h/4.0
            ),
            self.bb.w/2.0 +2.0*self.margin,
            self.bb.h/2.0 +2.0*self.margin,
            self.margin,
            self.capacity,
        );
        let mut se = QT::new(
            PT::new(
                self.bb.c.x - self.bb.w/4.0,
                self.bb.c.y - self.bb.h/4.0
            ),
            self.bb.w/2.0 +2.0*self.margin,
            self.bb.h/2.0 +2.0*self.margin,
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
        self.ne = El::Some(El::new_part(ne));
        self.se = El::Some(El::new_part(se));
        self.nw = El::Some(El::new_part(nw));
        self.sw = El::Some(El::new_part(sw));
    }
    fn directAdd(&mut self,o:PT) {
        if self.bb.contains(&o) {
            self.points.push(o.clone());
        }
    }
    fn return_rc(el: El<QT>, v: &mut Vec<Rc<RefCell<Box<QT>>>>) {
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
    println!("Hello, world!");
    let head = El::new_part(QT::new());
    // make a copy of head via rc clone,
    let mut walker = Rc::clone(&head);
    // then loop and add to it
    for i in 0..20 {
        // make a new Element then add it
        let tree_part = QT::new(i as f32);
        let new_ele = El::new_part(tree_part);
        let next_walker = Rc::clone(&new_ele);
        // replace the none at the end of walker
        walker.borrow_mut().ne = El::Some(new_ele);
        // make walker = new_ele
        walker = next_walker;
    }
    println!("result {:?}", head);
}
