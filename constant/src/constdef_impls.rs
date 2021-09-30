mod primitive;

mod std_heap {
    use crate::Constdef;

    impl<T> Constdef for Vec<T> {
        const DEFAULT: Self = new!();
    }

    impl Constdef for String {
        const DEFAULT: Self = new!();
    }
}
