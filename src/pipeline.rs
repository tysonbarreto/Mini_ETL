use serde_json::Map;




pub trait Transform<I,O>{
    fn transform(&self,input:I)->O;
}

pub struct MapTransform<F>{
    f:F
}

impl <F> MapTransform<F>{
    pub fn new(f:F)->Self{
        Self { f:f }
    }
}

impl <F,I,O> Transform<I,O> for MapTransform<F>
where
    F:Fn(I)->O
{
    fn transform(&self,input:I)->O {
        (self.f)(input)
    }
}   