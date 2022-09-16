use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

use av_data::frame::ArcFrame;
use av_data::packet::Packet;

use crate::common::CodecList2;
use crate::error::*;

/// Used to interact with a decoder.
pub trait Decoder: Send + Sync {
    /// Saves the extra data contained in a codec.
    fn set_extradata(&mut self, extra: &[u8]);
    /// Sends to the decoder a packet to be decoded.
    fn send_packet(&mut self, pkt: &Packet) -> Result<()>;
    /// Returns a decoded frame.
    fn receive_frame(&mut self) -> Result<ArcFrame>;
    /// Configures the decoder.
    fn configure(&mut self) -> Result<()>;
    /// Tells decoder to clear its internal state.
    fn flush(&mut self) -> Result<()>;
}

/// Used to save some data contained in a decoder.
pub trait SaveDecoderData<T> {
    /// Saves some data contained in a decoder.
    fn save_decoder_data(&self, context: &mut Context);
    ///
    fn get_data(&self) -> T;
}

/// Codec descriptor.
///
/// Contains information on a codec and its own decoder.
#[derive(Debug, PartialEq, Eq)]
pub struct Descr {
    /// The codec name.
    pub codec: &'static str,
    /// The extended codec name.
    pub name: &'static str,
    /// The codec description.
    pub desc: &'static str,
    /// The codec MIME.
    pub mime: &'static str,
    // TODO more fields regarding capabilities
}

/// Auxiliary structure to encapsulate a decoder object and
/// its additional data.
pub struct Context {
    dec: Box<dyn Decoder>,
    dec_data: Option<Arc<dyn Any + Send + Sync>>,
    // TODO: Queue up packets/frames
}

impl Context {
    // TODO: More constructors
    /*/// Creates the decoder associated to a codec descriptor and encapsulates
    /// it into a new `Context`.
    ///
    /// The codec descriptor is contained in a codec list and retrieved by its
    /// name.
    pub fn by_codecs(codecs: &Codecs, name: &str) -> Option<Self> {
        codecs.by_name(name).map(|builder| Context {
            dec: builder.create(),
        })
    }*/

    /// Adds a decoder.
    pub fn add_decoder<D: Decoder>(&mut self, dec: D) {
        self.dec = Box::new(dec);
    }

    /// Saves the extra data contained in a codec.
    pub fn set_extradata(&mut self, extra: &[u8]) {
        self.dec.set_extradata(extra);
    }

    /// Sends to the decoder a packet to be decoded.
    pub fn send_packet(&mut self, pkt: &Packet) -> Result<()> {
        self.dec.send_packet(pkt)
    }
    /// Returns a decoded frame.
    pub fn receive_frame(&mut self) -> Result<ArcFrame> {
        self.dec.receive_frame()
    }
    /// Configures the decoder.
    pub fn configure(&mut self) -> Result<()> {
        self.dec.configure()
    }

    /// Tells decoder to clear its internal state.
    pub fn flush(&mut self) -> Result<()> {
        self.dec.flush()
    }

    ///
    pub fn save_data(&mut self, data: impl SaveDecoderData) {
        self.dec_data = Arc::new(data.get_data());
    }
}

/// Used to get the descriptor of a codec and create its own decoder.
pub trait Descriptor {
    /// Creates a new decoder for the requested codec.
    fn create(&self, context: &mut Context);
    /// Returns the codec descriptor.
    fn describe(&self) -> &Descr;
}

/*/// A list of codec descriptors.
pub struct Codecs {
    list: HashMap<&'static str, Vec<Box<dyn Descriptor<Decoder = dyn Decoder>>>>,
}

impl CodecList2 for Codecs {
    type D = dyn Descriptor<Decoder = dyn Decoder>;

    fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }

    // TODO more lookup functions
    /*fn by_name(&self, name: &str) -> Option<&Box<&'static Self::D>> {
        self.list.get(name).map(|descs| &descs[0])
    }*/

    fn append(&mut self, desc: &'static Self::D) {
        let codec_name = desc.describe().codec;

        self.list
            .entry(codec_name)
            .or_insert_with(Vec::new)
            .push(desc);
    }
}*/

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! dec {
        ($mod_name:ident, $dec_name:expr, $descriptor:ident) => {
            mod $mod_name {
                use super::super::*;

                // A decoder is a private structure not accessible in a direct
                // way, only through a context.
                struct Dec {
                    state: usize,
                }

                impl Dec {
                    fn new(state: usize) -> Self {
                        Self { state }
                    }
                }

                /// A decoder descriptor.
                pub struct Des {
                    descr: Descr,
                }

                impl Descriptor for Des {
                    fn create(&self, context: &mut Context) {
                        context.add_decoder(Dec::new(0));
                    }

                    fn describe(&self) -> &Descr {
                        &self.descr
                    }
                }

                impl SaveDecoderData for Dec {
                    fn save_decoder_data(&self, context: &mut Context) {
                        context.save_data;
                    }
                }

                impl Decoder for Dec {
                    fn configure(&mut self) -> Result<()> {
                        Ok(())
                    }
                    fn set_extradata(&mut self, extra: &[u8]) {
                        if extra.len() > 4 {
                            self.state = 42;
                        } else {
                            self.state = 12;
                        }
                    }

                    fn send_packet(&mut self, _packet: &Packet) -> Result<()> {
                        self.state += 1;
                        Ok(())
                    }

                    fn receive_frame(&mut self) -> Result<ArcFrame> {
                        let yuv420 = *av_data::pixel::formats::YUV420;
                        let fm = std::sync::Arc::new(yuv420);
                        let video_info = av_data::frame::VideoInfo::new(
                            42,
                            42,
                            false,
                            av_data::frame::FrameType::I,
                            fm,
                        );
                        Ok(std::sync::Arc::new(
                            av_data::frame::Frame::new_default_frame(
                                av_data::frame::MediaKind::Video(video_info),
                                None,
                            ),
                        ))
                    }

                    fn flush(&mut self) -> Result<()> {
                        Ok(())
                    }
                }

                pub const $descriptor: &Des = &Des {
                    descr: Descr {
                        codec: $dec_name,
                        name: $dec_name,
                        desc: concat!($dec_name, "decoder"),
                        mime: concat!("x-application/", $dec_name),
                    },
                };
            }
            use self::$mod_name::$descriptor;
        };
    }

    dec!(dummy_dec, "dummy", DUMMY_DESCR);
    dec!(dummy_dec1, "dummy1", DUMMY_DESCR1);

    #[test]
    fn data_handling() {
      let context = Context::from_codecs(&codecs, name)
    }

    /*#[test]
    fn lookup_append() {
        let mut codecs = Codecs::new();
        codecs.append(DUMMY_DESCR);
        codecs.append(DUMMY_DESCR1);

        codecs.by_name("dummy").unwrap();
        codecs.by_name("dummy1").unwrap();
    }*/

    /*#[test]
    fn lookup_from_list() {
        let codecs = Codecs::from_list(&[DUMMY_DESCR, DUMMY_DESCR1]);

        codecs.by_name("dummy").unwrap();
        codecs.by_name("dummy1").unwrap();
    }

    #[test]
    #[should_panic]
    fn lookup_no_decoder() {
        let codecs = Codecs::from_list(&[DUMMY_DESCR, DUMMY_DESCR1]);

        codecs.by_name("dummy2").unwrap();
    }

    #[test]
    fn descriptor_data() {
        let codecs = Codecs::from_list(&[DUMMY_DESCR]);
        let descriptor = codecs.by_name("dummy").unwrap();
        assert_eq!(
            descriptor.describe(),
            &Descr {
                codec: "dummy",
                name: "dummy",
                desc: "dummy decoder",
                mime: "x-application/dummy",
            }
        )
    }

    #[test]
    fn context() {
        let codecs = Codecs::from_list(&[DUMMY_DESCR]);
        let packet = Packet::zeroed(10);

        let mut context = Context::by_codecs(&codecs, "dummy").unwrap();
        context.send_packet(&packet).unwrap();
    }*/
}
