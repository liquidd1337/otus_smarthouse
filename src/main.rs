struct SmartHouse {
    name: String,
    smart_kitchen: SmartRoom,
    smart_hall: SmartRoom,
    smart_living_room: SmartRoom,
    smart_bathroom: SmartRoom,
}

impl SmartHouse {
    fn new(
        house_name: String,
        kitchen_name: String,
        hall_name: String,
        living_room_name: String,
        bathroom_name: String,
        socket: SmartSocket,
        thermo: SmartThermometer,
    ) -> Self {
        SmartHouse {
            name: house_name,
            smart_kitchen: SmartRoom::default(kitchen_name, socket.clone(), thermo.clone()),
            smart_hall: SmartRoom::default(hall_name, socket.clone(), thermo.clone()),
            smart_living_room: SmartRoom::default(living_room_name, socket.clone(), thermo.clone()),
            smart_bathroom: SmartRoom::default(bathroom_name, socket.clone(), thermo.clone()),
        }
    }

    fn get_rooms(&self) -> [String; 4] {
        [
            self.smart_kitchen.room_name.clone(),
            self.smart_hall.room_name.clone(),
            self.smart_living_room.room_name.clone(),
            self.smart_bathroom.room_name.clone(),
        ]
    }

    fn devices(&self, room: &SmartRoom) -> String {
        format!(
            "Room : {} contains: {}, {}",
            room.room_name, room.smart_socket.name, room.smart_thermometr.name
        )
    }

    fn get_report(&self) -> String {
        format!(
        "In a house named: {0}, contains premises : {1:#?}
        \nEach of the rooms has the following devices:
        {2};
        {3};
        {4};
        {5};",
            self.name,
            self.get_rooms(),
            self.devices(&self.smart_kitchen),
            self.devices(&self.smart_hall),
            self.devices(&self.smart_living_room),
            self.devices(&self.smart_bathroom),
        )
    }
}

#[derive(Debug, Clone)]
struct SmartRoom {
    room_name: String,
    smart_socket: SmartSocket,
    smart_thermometr: SmartThermometer,
}

impl SmartRoom {
    fn default(
        room_name: String,
        smart_socket: SmartSocket,
        smart_thermometr: SmartThermometer,
    ) -> SmartRoom {
        SmartRoom {
            room_name: room_name.clone(),
            smart_socket: SmartSocket {
                name: format!("{} {}", room_name, smart_socket.name),
                status: smart_socket.status,
                voltage: smart_socket.voltage,
            },
            smart_thermometr: SmartThermometer {
                name: format!("{} {}", room_name, smart_thermometr.name),
                status: smart_thermometr.status,
                temperature: smart_thermometr.temperature,
            },
        }
    }
}
#[derive(Debug, Clone)]
struct SmartSocket {
    name: String,
    status: bool,
    voltage: f32,
}
#[derive(Debug, Clone)]
struct SmartThermometer {
    name: String,
    status: bool,
    temperature: f32,
}

fn main() {
    let socket1 = SmartSocket {
        name: "Smart socket".to_string(),
        status: true,
        voltage: 223.0,
    };

    let thermo = SmartThermometer {
        name: "Smart Thermometer".to_string(),
        status: true,
        temperature: 18.0,
    };

    let houses = SmartHouse::new(
        "HOUSE".to_string(),
        "Kitchen".to_string(),
        "Hall".to_string(),
        "Livingroom".to_string(),
        "Bathroom".to_string(),
        socket1,
        thermo,
    );

    let report = houses.get_report();
    println!("{}", report);
}
