use std::fmt::Debug;

fn main() {
    let my_condition: FilterCondition<u32> = FilterCondition { filter_type: (10) };
    let my_vector : Vec<u32> = vec![1,2,3,4,5,6,7,8,10];

    let my_filtered_list = custom_filter(my_condition, my_vector);
    print!("{:?}",my_filtered_list);
}


#[derive(Eq, PartialEq)]
struct FilterCondition<T: Eq + PartialOrd>{  
    filter_type: T,
}


impl<E:Eq + PartialOrd> FilterCondition<E>{
    fn is_match(&self,reference: &E) -> bool
    {
        if self.filter_type >= *reference
        {
            true
        }
        else
        {
            false
        }
    }
}


fn custom_filter<Z:Eq+Copy + PartialEq + PartialOrd,Y:Debug + IntoIterator<Item = Z>> (condition:FilterCondition<Z>,unfiltered_list: Y) -> Vec<Z>
{

    //let iterator = unfiltered_list.into_iter();
    /* 
    for item in iterator
    {
        if condition.is_match(item.unwrap())
        {
            item.
            filtered_list.push(*item);
        }
    }*/
    //condition.is_match(iterator.next());
    /*
    for item in iterator
    {
        println!("{:?}",dbg!(item));
    } */
    
    return unfiltered_list.into_iter().filter(|x| condition.is_match(x)).collect();

    //return filtered_list;
}