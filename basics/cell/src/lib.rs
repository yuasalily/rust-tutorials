use std::cell::{Cell, RefCell, UnsafeCell};

#[allow(dead_code)]
struct Immutables {
    cell: Cell<i32>,
    ref_cell: RefCell<i32>,
    unsafe_cell: UnsafeCell<i32>,
}

impl Default for Immutables {
    /// #[derive(Default)]と同じだけれど明示するために作成
    fn default() -> Self {
        Self {
            cell: Cell::new(0),
            ref_cell: RefCell::new(0),
            unsafe_cell: UnsafeCell::new(0),
        }
    }
}

#[cfg(test)]
mod cell_tests {

    use crate::Immutables;

    #[test]
    fn cell_test1() {
        let x = Immutables::default();

        let a = x.cell.get();
        let a2 = x.cell.get();
        assert_eq!(a, 0);
        assert_eq!(a2, 0);
        x.cell.set(1);
        let b = x.cell.get();
        assert_eq!(a2, 0);
        assert_eq!(b, 1);
    }

    #[test]
    fn cell_test2() {
        let mut x = Immutables::default();

        let a = x.cell.get_mut();
        assert_eq!(*a, 0);
        *a = 1;
        assert_eq!(*a, 1);
    }

    #[test]
    fn ref_cell_test() {
        let x = Immutables::default();

        let a = x.ref_cell.borrow();
        assert_eq!(*a, 0);
    }

    #[test]
    fn ref_cell_test2() {
        let x = Immutables::default();

        let mut a = x.ref_cell.borrow_mut();
        *a = 1;
        assert_eq!(*a, 1);
    }

    #[test]
    #[should_panic]
    fn ref_cell_test3() {
        let x = Immutables::default();

        let a = x.ref_cell.borrow();
        let mut b = x.ref_cell.borrow_mut();
    }

    #[test]
    fn unsafe_cell_test() {
        let x = Immutables::default();
        let y = x.unsafe_cell.get();
        unsafe {
            assert_eq!(*y, 0);
        }
    }
}
