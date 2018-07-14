pub struct List{
    list:Vec<i32>
}

impl List{
    pub fn new()-> List{
        List{
            list:vec!()
        }
    }
    
    pub fn push(&mut self,int:i32){
        self.list.push(int)
    }

    pub fn sort(&self,wall:i32,current_index:i32,pivot:i32){
        
    }
}
