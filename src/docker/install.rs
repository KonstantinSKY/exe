use crate::prelude::*;
use crate::linux::os;



pub fn run(){

    H1!("DOCKER SETUP");

    h2!("Checking if Docker is already installed...");
    exe!("docker --version; docker-compose --version"; true);

    h2!("Installing Docker & ECO System");
    os::install("docker docker-compose docker-buildx");
    
    exe!("docker --version; docker-compose --version"; true);
    
    h2!("Starting Docker Daemon systemclt");
    exe!("sudo systemctl start docker");
    exe!("sudo systemctl enable docker");
    
    h2!("Show Docker Status");
    exe!("systemctl status docker --no-pager"; true);
    
    
    h2!("Adding current user to docker group");
    exe!("sudo usermod -aG docker $USER");
    
    h2!("Don't forget system restart");
    exe!("sudo reboot");
    
}

pub fn hello(){
    h2!("Checking Docker by Hello world (test Image)");
    exe!("docker run hello-world");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
    }

}