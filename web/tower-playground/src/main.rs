impl Server {
    async fn run<F>(self, handler: F) -> Result<(), Error>
    where
        F: Fn(HttpRequest) -> HttpResponse,
    {
        let listener = TcpListener::bind(self.addr).await?;

        loop {
            let mut connection = listener.accept().await?;
            let request = read_http_request(&mut connection).await?;

            task::spawn(async move {
                // Call the handler provided by the user
                let response = handler(request);

                write_http_response(connection, response).await?;
            });
        }
    }
}

fn main() {
    println!("Jun's playground");
}