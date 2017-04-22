use std::fmt;
use std::cell::RefCell;
use std::cell;
use std::ops;
use std::slice;
use std::rc::Rc;
use std::ops::Index;
use port;

pub struct AudioBuffer{
    data: Vec<f32>
}

impl AudioBuffer{
    pub fn new(size: usize) -> AudioBuffer{
        AudioBuffer{ data: vec![0.0; size] }
    }
    pub fn iter(&self) -> AudioBufferIterator{
        AudioBufferIterator{ iter: self.data.iter() }
    }
    pub fn iter_mut(&mut self) -> AudioBufferIteratorMut{
        AudioBufferIteratorMut{ iter: self.data.iter_mut() }
    }
}

impl fmt::Debug for AudioBuffer{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = &self.data;
        write!(f, "AudioBuffer{{ /* {} samples */ }}", data.len())
    }
}

pub struct AudioBufferIterator<'a>{
    iter: slice::Iter<'a, f32>
}
pub struct AudioBufferIteratorMut<'a>{
    iter: slice::IterMut<'a, f32>
}


impl<'a> Iterator for AudioBufferIterator<'a>{
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        self.next()
    }
}
impl<'a> Iterator for AudioBufferIteratorMut<'a>{
    type Item = &'a mut f32;
    fn next(&mut self) -> Option<&'a mut f32> {
        self.next()
    }
}

#[derive(Debug)]
pub struct ReadBufferVector{
    vector: Vec<Rc<RefCell<AudioBuffer>>>
}

#[derive(Debug)]
pub struct WriteBufferVector{
    vector: Vec<Rc<RefCell<AudioBuffer>>>
}

struct AudioBufferRefWrapper<'a>{
    audiobuffer: cell::Ref<'a, AudioBuffer>
}

impl<'a> Iterator for AudioBufferRefWrapper<'a>{
    type Item = &'a f32;
    fn next(&mut self) -> Option<&'a f32> {
        self.next()
    }
}

impl ReadBufferVector{
    pub fn get<'a>(&'a self, port: port::Port) -> AudioBufferRefWrapper<'a>{
        let idx = port.nr;
        let audiobuffer = self.vector[idx].borrow();

        AudioBufferRefWrapper{ audiobuffer: audiobuffer }
    }
}

struct AudioBufferRefMutWrapper<'a>{
    audiobuffer: cell::RefMut<'a, AudioBuffer>
}

impl<'a> Iterator for AudioBufferRefMutWrapper<'a>{
    type Item = &'a mut f32;
    fn next(&mut self) -> Option<&'a mut f32>{
        self.next()
    }
}

impl WriteBufferVector{
    pub fn get<'a>(&'a self, port: port::Port) -> AudioBufferRefMutWrapper<'a>{
        let idx = port.nr;
        let audiobuffer = self.vector[idx].borrow_mut();
        AudioBufferRefMutWrapper{  audiobuffer: audiobuffer }
    }
}
