// Print and format macros
mod geo_map;

fn main () {
  let my_favorite_place = geo_map::get_hawaii_loation();

  println!("My favorite place is lat {} and long {}", my_favorite_place.lat, my_favorite_place.long);
}