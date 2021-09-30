mod primitive;

mod alloc_crate {
    use crate::Constdef;

    impl<T> Constdef for Vec<T> {
        const DEFAULT: Self = new!();
    }

    impl Constdef for String {
        const DEFAULT: Self = new!();
    }
}

mod std_collections {
    use crate::Constdef;
    use std::collections::LinkedList;
    impl<T> Constdef for LinkedList<T> {
        const DEFAULT: Self = LinkedList::new();
    }
}

mod core_mem {
    use crate::Constdef;
    use core::mem::MaybeUninit;
    impl<T> Constdef for MaybeUninit<T> {
        const DEFAULT: Self = Self::uninit();
    }
}
