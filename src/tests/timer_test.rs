#[cfg(test)]
mod tests {
    use super::*;
    use crate::timer::Timer;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};
    use std::thread;
    
    #[test]
    fn test_timer_execution() {
        let executed = Arc::new(Mutex::new(false));
        let executed_clone = Arc::clone(&executed);
        
        let timer = Timer::new(Duration::from_millis(50), Box::new(move || {
            *executed_clone.lock().unwrap() = true;
        }));
        
        timer.start();
        thread::sleep(Duration::from_millis(100)); // Allow time for execution
        
        assert!(*executed.lock().unwrap());
    }
    
    #[test]
    fn test_timer_stop() {
        let executed = Arc::new(Mutex::new(0));
        let executed_clone = Arc::clone(&executed);
        
        let timer = Timer::new(Duration::from_millis(50), Box::new(move || {
            *executed_clone.lock().unwrap() += 1;
        }));
        
        timer.start();
        thread::sleep(Duration::from_millis(120));
        timer.stop();
        let count_after_stop = *executed.lock().unwrap();
        thread::sleep(Duration::from_millis(100)); // Wait longer to ensure no execution
        
        assert_eq!(*executed.lock().unwrap(), count_after_stop);
    }
}
