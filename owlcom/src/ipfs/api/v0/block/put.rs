pub struct Put{
    args:String,
}
impl Put{
    pub fn builder()->Builder{
        Builder::default()
    }
}

#[derive(Default)]
pub struct Builder{
    changed:bool,
    opt_args:String
}
impl Builder{
    pub fn cid_codec(self,cid_codec:String)->Self{
        if self.changed{
            Self{
                
            }
        }
    }
}