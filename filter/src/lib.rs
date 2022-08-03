// Each filter MUST return a node in order to be assembled in some way



/// Video information for filtering.
pub struct VideoInfo {
    /// All information related to a video
    vi: av_data::VideoInfo,
    /// Number of frames of a video
    num_frames: u32,
}

/// A mechanism to create a filter.
pub trait Filter {
   /// Creates a filter from a series of internal parameters.
   ///
   /// It returns a `Node`.
   fn create_filter(&self) -> Result<Node>;
}



/* void VSCore::createVideoFilter(VSMap *out, const std::string &name, const VSVideoInfo *vi, VSFilterGetFrame getFrame, VSFilterFree free, VSFilterMode filterMode, const VSFilterDependency *dependencies, int numDeps, void *instanceData, int apiMajor) {
    try {
        VSNode *node = new VSNode(name, vi, getFrame, free, filterMode, dependencies, numDeps, instanceData, apiMajor, this);
        vs_internal_vsapi.mapConsumeNode(out, "clip", node, maAppend);
    } catch (VSException &e) {
        vs_internal_vsapi.mapSetError(out, e.what());
    }
}*/

// Video filterchain:
//
// - Create a new filter with `new` function in order to set parameters
// - Use `create_filter` to transform a filter in a Node.
// - Use set_output to print a Node


// Node:
// - Discriminate among a VideoNode and an AudioNode
// - Provide a function to return the output as y4m file

// A video node
pub struct VideoNode;

trait OutputNode {
    fn output(&self) -> Result<()>;
}

// BLANKCLIP (move in another file afterwards)

/// A simple blank clip video
pub struct BlankClip {
    frame: av_data::Frame,
    vi: VideoInfo,
    color: [u32, 3],
    keep: bool,
}


//pub fn new_default_frame<T>(kind: T, t: Option<TimeInfo>) -> Self



#[cfg(test)]
mod test {
    use super::Node;

    use BlankClip

    #[test]
    fn simple_video_filter() {

        // Creates a blank clip gettin in output a Node
        let video: Node = BlankClip::new(1280, 720).unwrap();

        //

        //
    }
}
