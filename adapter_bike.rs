// Определяем интерфейс для велосипедов
trait Bike {
    // Метод, который должен реализовать каждый велосипед
    fn ride(&self);
}

// Реализуем структуру для шоссейного велосипеда
struct RoadBike;

// Реализация интерфейса для шоссейного велосипеда
impl Bike for RoadBike {
    // Метод, специфичный для шоссейного велосипеда
    fn ride(&self) {
        println!("Еду на шоссейном велосипеде!");
    }
}

// Реализуем структуру для гравийного велосипеда
struct GravelBike;

impl GravelBike {
    // Метод, специфичный для гравийного велосипеда
    fn ride_gravel(&self) {
        println!("Еду на гравийном велосипеде!");
    }
}

// Адаптер для гравийного велосипеда, чтобы он соответствовал интерфейсу Bike
struct GravelBikeAdapter {
    gravel_bike: GravelBike, // Внутренняя структура гравийного велосипеда
}

// Реализация интерфейса Bike для адаптера
impl Bike for GravelBikeAdapter {
    fn ride(&self) {
        // Перенаправление вызова метода ride на метод гравийного велосипеда
        self.gravel_bike.ride_gravel();
    }
}

fn main() {
    // Создаем экземпляр шоссейного велосипеда
    let road_bike = RoadBike;
    road_bike.ride(); // Вызов метода ride для шоссейного велосипеда

    // Создаем экземпляр гравийного велосипеда
    let gravel_bike = GravelBike;
    // Создаем адаптер для гравийного велосипеда
    let gravel_adapter = GravelBikeAdapter { gravel_bike };
    gravel_adapter.ride(); // Вызов метода ride для адаптированного гравийного велосипеда
}
