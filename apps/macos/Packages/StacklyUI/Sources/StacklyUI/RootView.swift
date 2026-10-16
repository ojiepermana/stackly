import SwiftUI

/// The content of the main Stackly window.
///
/// A placeholder until the app shell is built.
public struct RootView: View {
    /// Creates the root view.
    public init() {}

    /// The view's content.
    public var body: some View {
        ContentUnavailableView("Stackly", systemImage: "square.stack.3d.up")
    }
}

#Preview {
    RootView()
}
