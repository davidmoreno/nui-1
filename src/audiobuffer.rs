use std::fmt;
use std::cell::RefCell;
use std::cell::RefMut;
use std::cell::Ref;
use std::rc::Rc;
use port;
use std::iter::IntoIterator;
use std::clone::Clone;
use std::iter::Zip;

pub struct AudioBuffer{
    data: Vec<f32>
}

impl AudioBuffer{
    pub fn new(size: usize) -> AudioBuffer{
        AudioBuffer{ data: vec![0.0; size] }
    }
}

impl IntoIterator for AudioBuffer{
    type Item = f32;
    type IntoIter = ::std::vec::IntoIter<f32>;
    fn into_iter(self) -> Self::IntoIter{
        self.data.into_iter()
    }
}

impl<'a> IntoIterator for &'a mut AudioBuffer{
    type Item = &'a mut f32;
    type IntoIter = ::std::slice::IterMut<'a, f32>;
    fn into_iter(self) -> Self::IntoIter{
        self.data.iter_mut()
    }
}

impl<'a> IntoIterator for &'a AudioBuffer{
    type Item = &'a f32;
    type IntoIter = ::std::slice::Iter<'a, f32>;
    fn into_iter(self) -> Self::IntoIter{
        self.data.iter()
    }
}

impl fmt::Debug for AudioBuffer{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = &self.data;
        write!(f, "AudioBuffer{{ /* {} samples */ }}", data.len())
    }
}

#[derive(Debug)]
pub struct AudioBufferRef{
    audiobuffer: Rc<RefCell<AudioBuffer>>
}

impl AudioBufferRef{
    pub fn new(size: usize) -> AudioBufferRef{
        let audiobuffer = Rc::new(RefCell::new(AudioBuffer::new(size)));
        AudioBufferRef{audiobuffer: audiobuffer}
    }
}

impl Clone for AudioBufferRef{
    fn clone(&self) -> AudioBufferRef{
        AudioBufferRef{ audiobuffer: self.audiobuffer.clone() }
    }
}

#[derive(Debug)]
pub struct ROAudioBuffer<'a>{
    audiobuffer: Ref<'a, AudioBuffer>
}

pub struct ROAudioBufferIterator<'a>{
    audiobuffer: Ref<'a, AudioBuffer>,
    position: usize
}

impl<'a> IntoIterator for ROAudioBuffer<'a>{
    type Item = f32;
    type IntoIter = ROAudioBufferIterator<'a>;
    fn into_iter(self) -> Self::IntoIter{
        ROAudioBufferIterator{audiobuffer: self.audiobuffer, position: 0}
    }
}

impl<'a> Iterator for ROAudioBufferIterator<'a>{
    type Item = f32;
    #[inline]
    fn next(&mut self) -> Option<f32> {
        let ret = match self.audiobuffer.data.get(self.position) {
            Some(v) => Some(*v),
            None => None
        };
        self.position+=1;
        ret
    }
}

#[derive(Debug)]
pub struct RWAudioBuffer<'a>{
    audiobuffer: RefMut<'a, AudioBuffer>
}

#[derive(Debug)]
pub struct AudioBufferVector{
    vector: Vec<AudioBufferRef>
}

impl AudioBufferVector{
    pub fn get(&self, port: port::Port) -> ROAudioBuffer{
        let idx = port.nr;
        let audiobuffer = (*self.vector[idx].audiobuffer).borrow();

        ROAudioBuffer{ audiobuffer: audiobuffer }
    }
    pub fn get_mut(&self, port: port::Port) -> RWAudioBuffer{
        let idx = port.nr;
        let audiobuffer = self.vector[idx].audiobuffer.borrow_mut();

        RWAudioBuffer{ audiobuffer: audiobuffer }
    }
    pub fn get_ref(&self, port: port::Port) -> AudioBufferRef{
        let idx = port.nr;
        let audiobuffer = self.vector[idx].clone();

        audiobuffer
    }
    pub fn set_ref(&mut self, port: port::Port, audiobuffer: AudioBufferRef){
        self.vector[port.nr] = audiobuffer
    }
    pub fn new(count: usize, size: usize) -> AudioBufferVector{
        let mut vector = Vec::new();
        for _i in 0..count{
            vector.push(AudioBufferRef::new(size));
        }
        AudioBufferVector{ vector: vector }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn audiobuffer_as_iterator(){
        let mut ab = AudioBuffer::new(128);
        for o in &mut ab{
            *o = 1.0;
        }
        for i in &ab{
            assert!(*i == 1.0 );
        }
        for i in ab{ // This moves the ab
            assert!(i == 1.0 );
        }
    }

    #[test]
    fn robuffervector(){
        let rbv = AudioBufferVector::new(3, 128);

        // Now borrow 1 as RO
        {
            let r = rbv.get(port::Port::new(1));
            for i in r{
                assert_eq!(i, 0.0);
            }
        }
    }

    // #[test]
    // fn rwbuffervector(){
    //     let rbv = AudioBufferVector::new(3, 128);
    //
    //     // First read and write on diferent buffers, must be refcell unborrowed at end
    //     {
    //         let r = rbv.get(port::Port::new(0));
    //         let w = rbv.get_mut(port::Port::new(1));
    //
    //         for (o, i) in Zip::zip(r,w) {
    //             *o = i*2.0
    //         }
    //     }
    //     // Now borrow 1 as RO
    //     {
    //         let r = rbv.get(port::Port::new(1));
    //         for i in r{
    //             assert!(i == 256.0);
    //         }
    //     }
    // }
}
