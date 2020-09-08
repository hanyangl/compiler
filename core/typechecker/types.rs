use sflyn_parser::tokens::Token;

pub fn check_types(
  one: Token,
  two: Token,
  from_array: bool,
) -> bool {
  // Check if the one token is a valid data type.
  if let Some(one_type) = one.token.clone().get_type() {
    // Check if the two token is a valid data type.
    if let Some(two_type) = two.token.clone().get_type() {
      // Check if they are groups.
      if one_type.clone().get_group().is_some() && two_type.clone().get_group().is_some() {
        let mut one_group = one_type.clone().get_group().unwrap();
        let mut two_group = two_type.clone().get_group().unwrap();
        let mut return_v = false;

        // Sort alphabetically the first group of types.
        one_group.types.sort_by(
          |a, b| a.value.to_lowercase().cmp(&b.value.to_lowercase())
        );

        // Sort alphabetically the second group of types.
        two_group.types.sort_by(
          |a, b| a.value.to_lowercase().cmp(&b.value.to_lowercase())
        );

        let mut index: usize = 0;

        if one_group.types.len() <= two_group.types.len() {
          for data_type in one_group.types.iter() {
            let result = check_types(data_type.clone(), two_group.types[index].clone(), from_array);

            if result && !from_array {
              return true;
            } else if from_array {
              if !result {
                return false;
              }

              return_v = true;
            }

            index += 1;
          }
        } else {
          for data_type in two_group.types.iter() {
            let result = check_types(data_type.clone(), one_group.types[index].clone(), from_array);

            if result && !from_array {
              return true;
            } else if from_array {
              return_v = true;
            }

            index += 1;
          }
        }

        return return_v;
      }
      // Check if the one type is a group.
      else if let Some(one_group) = one_type.clone().get_group() {
        let mut return_v = false;

        if one_group.types.len() > 0 {
          for data_type in one_group.types.iter() {
            let result = check_types(data_type.clone(), two.clone(), from_array);

            if result && !from_array {
              return true;
            } else if from_array {
              if !result {
                return false;
              }

              return_v = true;
            }
          }
        }

        return return_v;
      }
      // Check if the two type is a group.
      else if let Some(two_group) = two_type.clone().get_group() {
        let mut return_v = false;

        if two_group.types.len() > 0 {
          for data_type in two_group.types.iter() {
            let result = check_types(data_type.clone(), one.clone(), from_array);

            if result && !from_array {
              return true;
            } else if from_array {
              if !result {
                return false;
              }

              return_v = true;
            }
          }
        }

        return return_v;
      }
      // Check if they are arrays.
      else if one_type.clone().get_array().is_some() && two_type.clone().get_array().is_some() {
        let one_array = one_type.clone().get_array().unwrap();
        let two_array = two_type.clone().get_array().unwrap();

        return check_types(one_array.data_type, two_array.data_type, true);
      }
      // Check if the one type is an array.
      else if let Some(one_array) = one_type.clone().get_array() {
        return check_types(one_array.data_type, two, true);
      }
      // Check if the two type is an array.
      else if let Some(two_array) = two_type.clone().get_array() {
        return check_types(two_array.data_type, one, true);
      }

      return one_type == two_type;
    }
  }

  false
}

pub fn equal_type_and_interface(data_type: Token, interface: Token) -> bool {
  // Get the data type.
  if let Some(data_type) = data_type.token.clone().get_type() {
    // Get the hashmap data type.
    if let Some(data_type_hashmap) = data_type.clone().get_hashmap() {
      // Get the interface data type.
      if let Some(interface_type) = interface.token.clone().get_type() {
        // Get the interface hashmap.
        if let Some(interface_hashmap) = interface_type.clone().get_hashmap() {
          for (key, value) in interface_hashmap.items {
            if let Some(item_type) = data_type_hashmap.items.get(&key) {
              if !check_types(value, item_type.clone(), false) {
                return false;
              }

              continue;
            }

            return false;
          }

          return true;
        }
      }
    }
  }

  false
}
