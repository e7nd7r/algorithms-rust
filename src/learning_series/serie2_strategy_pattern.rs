use std::cmp::Ordering;

#[derive(PartialEq)]
pub enum SortBy {
    Name,
    Age
}

#[derive(PartialEq, Debug)]
pub struct Person {
    pub name: String,
    pub age: i32
}

pub fn quick_sort<T: PartialOrd>(collection: &mut [T]) {
    if collection.len() <= 1 {
        return;
    }

    let pivot_index = partition(collection);

    quick_sort(&mut collection[0 .. pivot_index]);
    quick_sort(&mut collection[pivot_index + 1 ..])
}

fn partition<T: PartialOrd>(collection: &mut [T]) -> usize {
    let len = collection.len();
    let pivot_index =  len / 2;

    collection.swap(pivot_index, len - 1);

    let mut store_index = 0;

    for i in 0 .. len - 1 {
        if collection[i] < collection[len - 1] {
            collection.swap(i, store_index);
            store_index += 1;
        }
    }

    collection.swap(store_index, len - 1);

    store_index
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.to_lowercase().partial_cmp(&other.name.to_lowercase())        
    }
}

#[cfg(test)] 
mod tests {
    use crate::learning_series::serie2_strategy_pattern::*;

    #[test]
    pub fn test_sort() {
        let mut persons = [
            Person {
                name: String::from("esteban"),
                age: 34,
            },
            Person {
                name: String::from("Noemie"),
                age: 36,
            },
            Person {
                name: String::from("Sofia"),
                age: 21,
            },
            Person {
                name: String::from("Edgar"),
                age: 23,
            }
        ];

        quick_sort(&mut persons);

        println!("{:?}", persons)
    }
}