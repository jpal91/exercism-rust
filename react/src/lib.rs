#![allow(unused)]
use std::collections::HashMap;
use std::io::ErrorKind;
use std::time::Instant;

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId<T>(T);

/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ComputeCellId<T>(T);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CallbackId(Instant);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CellId<T> {
    Input(InputCellId<T>),
    Compute(ComputeCellId<T>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}


pub struct CallBack<'a, T> {
    id: CallbackId,
    comp_id: ComputeCellId<T>,
    callbacks: Vec<InputCellId<T>>,
    func: Box<dyn FnMut(T) + 'a>,
    cur_val: T
}

type ComputeInput<T> = (Vec<CellId<T>>, Box<dyn Fn(&[T]) -> T>);
type CallBackInput<T> = (CellId<T>, Box<dyn FnMut(T)>);

pub struct Reactor<'a, T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    pub inp_map: HashMap<CellId<T>, T>,
    pub comp_map: HashMap<CellId<T>, ComputeInput<T>>,
    pub cb: Vec<CallBack<'a, T>>
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq + Eq + std::hash::Hash + std::fmt::Debug> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            inp_map: HashMap::new(),
            comp_map: HashMap::new(),
            cb: vec![]
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, _initial: T) -> InputCellId<T> {
        let input = InputCellId(_initial);
        self.inp_map.insert(CellId::Input(input), _initial); 
        input
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(
        &mut self,
        _dependencies: &[CellId<T>],
        _compute_func: F,
    ) -> Result<ComputeCellId<T>, CellId<T>> {
        let mut vals: Vec<T> = vec![];

        for d in _dependencies.iter() {
            let val = match self.value(*d) {
                Some(v) => v,
                None => return Err(d.to_owned())
            };
            vals.push(val)
        };

        let id = ComputeCellId(_compute_func(&vals));
        self.comp_map.insert(CellId::Compute(id), (_dependencies.to_owned(), Box::new(_compute_func)));
        Ok(id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId<T>) -> Option<T> {
        match id {
            c @ CellId::Input(_) => self.inp_map.get(&c).copied(),
            c @ CellId::Compute(_) => {
                if let Some(_c) = self.comp_map.get(&c) {
                    Some(self.compute_value(_c))
                } else {
                    None
                }
            }
        }
    }

    fn compute_value(&self, input: &ComputeInput<T>) -> T {
        let mut inputs: Vec<T> = vec![];

        for inp in input.0.iter() {
            let _i = match inp {
                c @ CellId::Input(_) => self.inp_map.get(c).unwrap().to_owned(),
                c @ CellId::Compute(_) => self.value(c.to_owned()).unwrap()
            };
            inputs.push(_i)
        }
        let func: &dyn Fn(&[T]) -> T = &input.1;
        func(&inputs)
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, _id: InputCellId<T>, _new_value: T) -> bool {
        let key = CellId::Input(_id);
        
        if !self.inp_map.contains_key(&key) {
            false
        } else {
            self.inp_map.insert(CellId::Input(_id), _new_value);
            self.check_callbacks(_id);
            true
        }
    }

    fn check_callbacks(&mut self, id: InputCellId<T>) {
        // let callbacks: Vec<_> = self.cb.iter()
        //     .enumerate()
        //     .filter_map(|(i, cb)| cb.contains(&id).map(|cv| (i, cv)))
        //     .collect();

        // for (i, cv) in callbacks {
        //     let new_val = self.value(CellId::Compute(cv)).unwrap();
        //     self.cb[i].callback(new_val)
        // }
        
        for i in 0..self.cb.len() {
            let new_val = self.value(CellId::Compute(self.cb[i].comp_id.clone())).unwrap();
            
            if new_val != self.cb[i].cur_val {
                self.cb[i].callback(new_val);
                self.cb[i].cur_val = new_val;
            }
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        _id: ComputeCellId<T>,
        _callback: F,
    ) -> Option<CallbackId> {
        let callbacks = if let Some(c) = self.comp_map.get(&CellId::Compute(_id)) {
            self.find_inp_cells(c)
        } else {
            return None
        };

        let id = CallbackId(Instant::now());
        let cb = CallBack {
            id,
            comp_id: _id,
            callbacks,
            func: Box::new(_callback),
            cur_val: self.value(CellId::Compute(_id)).unwrap()
        };

        self.cb.push(cb);

        Some(id)
    }

    fn find_inp_cells(&self, input: &ComputeInput<T>) -> Vec<InputCellId<T>> {
        let mut res: Vec<InputCellId<T>> = vec![];

        for cell in input.0.iter() {
            match cell {
                CellId::Input(c) => res.push(c.to_owned()),
                c @ CellId::Compute(_) => {
                    let c_cells = self.comp_map.get(c).unwrap();
                    let embedded = self.find_inp_cells(c_cells);
                    res.extend(embedded)
                }
            }
        }
        println!("{:?}", res);
        res
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId<T>,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if !self.comp_map.contains_key(&CellId::Compute(cell)) {
            return Err(RemoveCallbackError::NonexistentCell)
        }

        let mut idx: Option<usize> = None;
        for (i, cb) in self.cb.iter().enumerate() {
            if cell == cb.comp_id && callback == cb.id {
                idx = Some(i);
                break
            }
        };

        if let Some(i) = idx {
            self.cb.remove(i);
            Ok(())
        } else {
            Err(RemoveCallbackError::NonexistentCallback)
        }
        
    }
}

impl<'a, T: PartialEq + Clone + std::fmt::Debug> CallBack<'a, T> {
    fn contains(&self, inp: &InputCellId<T>) -> Option<ComputeCellId<T>> {
        println!("Callback - {:?} {:?}", self.callbacks, self.comp_id);
        if self.callbacks.contains(inp) {
            Some(self.comp_id.clone())
        } else {
            None
        }
    }

    fn callback(&mut self, inp: T) {
        let func = &mut self.func;
        func(inp);
    }
}