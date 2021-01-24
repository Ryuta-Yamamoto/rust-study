provider "google" {
    project = "sunny-furnace-302710"
    credentials = "${file("../credentials/gcp")}"
    region = "asia-northeast1"
}

resource "google_compute_instance" "default" {
    name         = "test"
    machine_type = "e2-micro"
    zone         = "asia-northeast1-a"
  
    boot_disk {
        initialize_params {
            image = "debian-cloud/debian-9"
        }
    }

    network_interface {
        network = "default"
    }
}
