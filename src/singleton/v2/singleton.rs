#[macro_export]
macro_rules! Singleton {
    ($t: ty,$name: ident, $e: expr) => {
        pub mod $name {
            use std::mem::MaybeUninit;
            use std::sync::Mutex;
            use std::sync::Once;

            static mut SINGLETON: MaybeUninit<Singleton> = MaybeUninit::uninit();
            static ONCE: Once = Once::new();
            static mut INIT: fn() -> $t = $e;

            pub struct Singleton {
                inner: Mutex<$t>,
            }

            impl Singleton {
                pub fn inner(&mut self) -> &mut Mutex<$t> {
                    &mut self.inner
                }
            }

            pub fn instance() -> &'static mut Singleton {
                ONCE.call_once(|| {
                    let init = unsafe { INIT };
                    let value = init();

                    let singleton = Singleton {
                        inner: Mutex::new(value),
                    };

                    unsafe {
                        SINGLETON.write(singleton);
                    }
                });
                unsafe { SINGLETON.assume_init_mut() }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    Singleton!(u32, singletonu32, || 42);
    Singleton!(u64, singletonu64, || 42);
    Singleton!(f64, singletonf64, || 42.);

    #[test]
    fn singleton() {
        let instance1 = singletonu32::instance();
        let instance2 = singletonu32::instance();

        assert_eq!(*instance1.inner().lock().unwrap(), 42);
        assert_eq!(*instance2.inner().lock().unwrap(), 42);

        *instance1.inner().get_mut().unwrap() = 24;

        assert_eq!(*instance2.inner().lock().unwrap(), 24);

        *instance2.inner().get_mut().unwrap() = 142;

        assert_eq!(*instance1.inner().lock().unwrap(), 142);
    }

    #[test]
    fn singleton_multi_instance() {
        let instance1 = singletonu64::instance();
        let instance2 = singletonu64::instance();
        let instance3 = singletonf64::instance();

        assert_eq!(*instance1.inner().lock().unwrap(), 42);
        assert_eq!(*instance2.inner().lock().unwrap(), 42);
        assert_eq!(*instance3.inner().lock().unwrap(), 42.);

        *instance3.inner().get_mut().unwrap() = 24.;
        *instance1.inner().get_mut().unwrap() = 24;

        assert_eq!(*instance2.inner().lock().unwrap(), 24);

        *instance2.inner().get_mut().unwrap() = 142;

        assert_eq!(*instance1.inner().lock().unwrap(), 142);
        drop(instance1);
        assert_eq!(*instance2.inner().lock().unwrap(), 142);
        assert_eq!(*instance3.inner().lock().unwrap(), 24.);
    }
}
