#[derive(Debug)]
pub struct List{
    list:Vec<i32>
}

impl List{
    pub fn new()-> List{
        List{
            list:vec!()
        }
    }

    pub fn new_list(list:Vec<i32>)->List{
        List{
            list:list,
        }
    }
    pub fn push(&mut self,int:i32){
        self.list.push(int)
    }

    pub fn sort(&mut self,low:i32,high:i32){
        if low < high{
            let pi = self.partition(low,high);
            self.sort(low,pi-1);
            self.sort(pi+1,high);
        }
    }
    
    pub fn get_len(&self)->i32{
        self.list.len() as i32
    }
    
    fn partition(&mut self,low:i32,high:i32)->i32{
        let pivot = self.list.get(high as usize).unwrap().clone();
        let mut i = low-1;
        let mut j = low as usize;
        loop{
            if self.list.get(j).unwrap() <= &pivot{
                i+=1;
                self.list.swap(i as usize,j);
            }
            if j == (high-1) as usize{
                break;
            }
            j+=1;
        }
        i+=1;
      self.list.swap(i as usize,high as usize);
      return i;
    }
    
    pub fn clone(&self)->List{
        List{
            list:self.list.clone()
        }
    }

    pub fn write(&self){
        println!("{:?}",self.list);
    }
}
