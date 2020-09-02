use sflyn_parser::tokens::Token;

pub fn equal_types(
  one: Token,
  two: Token,
) -> bool {
  // Get the one data type.
  if let Some(one_type) = one.token.clone().get_type() {
    // Get the two data type.
    if let Some(two_type) = two.token.clone().get_type() {
      // Check if they are arrays.
      if one_type.clone().get_array().is_some() && two_type.clone().get_array().is_some() {
        // TODO
      }
      // Check if they are functions.
      else if one_type.clone().get_function().is_some() && two_type.clone().get_function().is_some() {
        // TODO
      }
      // Check if they are groups.
      else if one_type.clone().get_group().is_some() && two_type.clone().get_group().is_some() {
        return one_type.clone().get_group().unwrap().get_types_strings() == two_type.clone().get_group().unwrap().get_types_strings();
      }
      // Check if one is a group.
      else if let Some(one_group) = one_type.clone().get_group() {
        return one_group.has_type(two.value);
      }
      // Check if two is a group.
      else if let Some(two_group) = two_type.clone().get_group() {
        return two_group.has_type(one.value);
      }
      // Check if they are hashmaps.
      else if one_type.clone().get_hashmap().is_some() && two_type.clone().get_hashmap().is_some() {
        // TODO
      }

      return one_type == two_type;
    }
  }

  false
}

pub fn equal_type_and_interface(
  data_type: Token,
  interface: Token,
) -> bool {
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
              if !equal_types(value, item_type.clone()) {
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
