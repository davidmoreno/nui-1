use std::fmt;
use port;
use std::iter::IntoIterator;

pub struct AudioBuffer{
    data: Box<Vec<f32>>
}

impl AudioBuffer{
    pub fn new(size: usize) -> AudioBuffer{
        AudioBuffer{ data: Box::new(vec![0.0; size]) }
    }
}

// impl IntoIterator for AudioBuffer{
//     type Item = f32;
//     type IntoIter = ::std::vec::IntoIter<f32>;
//     fn into_iter(self) -> Self::IntoIter{
//         self.data.into_iter()
//     }
// }

impl<'a> IntoIterator for &'a AudioBuffer{
    type Item = &'a f32;
    type IntoIter = ::std::slice::Iter<'a, f32>;
    fn into_iter(self) -> Self::IntoIter{
        self.data.iter()
    }
}


impl<'a> IntoIterator for &'a mut AudioBuffer{
    type Item = &'a mut f32;
    type IntoIter = ::std::slice::IterMut<'a, f32>;
    fn into_iter(self) -> Self::IntoIter{
        self.data.iter_mut()
    }
}

impl fmt::Debug for AudioBuffer{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = &self.data;
        write!(f, "AudioBuffer{{ /* {} samples */ }}", data.len())
    }
}

impl fmt::Display for AudioBuffer{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for s in self.into_iter(){
            write!(f, "{} ", s);
        }
        write!(f, "")
    }
}


#[derive(Debug)]
pub struct AudioBufferVector{
    vector: Vec<Option<AudioBuffer>>
}

impl AudioBufferVector{
    pub fn get(&mut self, idx: usize) -> Option<AudioBuffer>{
        self.vector.get_mut(idx)
            .and_then(|ab| ab.take())
    }
    // pub fn get_mut(&self, port: port::Port) -> RWAudioBuffer{
    //     let idx = port.nr;
    //     RWAudioBuffer{ audiobuffer: self.vector[idx] }
    // }
    pub fn put(&mut self, idx: usize, audiobuffer: AudioBuffer){
        self.vector[idx] = Some(audiobuffer)
    }
    pub fn new(count: usize, size: usize) -> AudioBufferVector{
        let mut vector = Vec::new();
        for _i in 0..count{
            vector.push(Some(AudioBuffer::new(size)));
        }
        AudioBufferVector{ vector: vector }
    }
    pub fn new_empty(count: usize) -> AudioBufferVector{
        let mut vector = Vec::new();
        for _i in 0..count{
            vector.push(None);
        }
        AudioBufferVector{ vector: vector }
    }
    pub fn check_all_some(&self) -> bool{
        for ab in &self.vector{
            if !ab.is_some() {
                return false;
            }
        }
        true
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
    }

    #[test]
    fn robuffervector(){
        let mut rbv = AudioBufferVector::new(3, 128);

        // Now borrow 1 as RO
        {
            let r = rbv.get(1).unwrap();
            for i in &r{
                assert_eq!(*i, 0.0);
            }
        }
    }

    #[test]
    fn rwbuffervector(){
        let mut rbv = AudioBufferVector::new(3, 128);

        // First read and write on diferent buffers, must be refcell unborrowed at end
        {
            let r = rbv.get(0).unwrap();
            let mut w = rbv.get(1).unwrap();

            for o in &mut w {
                *o = 128.0;
            }

            rbv.put(0, r);
            rbv.put(1, w);
        }
        // Now borrow 1 as RO
        {
            let r = rbv.get(1).unwrap();
            for i in &r{
                assert!(*i == 128.0);
            }
            rbv.put(1, r);
        }
    }

    #[test]
    fn zipbuffervector(){
        let mut rbv = AudioBufferVector::new(3, 128);

        // First read and write on diferent buffers, must be refcell unborrowed at end
        {
            let r = rbv.get(0).unwrap();
            let mut w = rbv.get(1).unwrap();

            for (o, i) in ::itertools::zip(&mut w, &r) {
                *o = i + 128.0;
            }

            rbv.put(0, r);
            rbv.put(1, w);
        }
        // Now borrow 1 as RO
        {
            let r = rbv.get(1).unwrap();
            for i in &r{
                assert!(*i == 128.0);
            }
            rbv.put(1, r);
        }
    }
}
