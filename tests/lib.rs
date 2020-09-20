#[cfg(test)]
mod tests {
    use express_rs::*;
    use std::sync::mpsc::channel;

    #[test]
    #[should_panic]
    fn port_out_of_range_min() {
        let mut app = Express::new();
        app.listen(0)
    }

    #[test]
    fn process_running() {
        let (tx, rx) = channel();

        std::thread::spawn(move || {
            let mut app = Express::new();
            app.listen(65535);

            tx.send("Thread exited").unwrap()
        });

        // If a value is received in less than 1 second, we can assume that the process has finished running.
        let did_exit = rx.recv_timeout(std::time::Duration::from_secs(1));

        // Assert that the process did not exit
        assert_eq!(did_exit.is_ok(), false);
    }
}
