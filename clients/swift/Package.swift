// swift-tools-version: 6.4
import PackageDescription

let package = Package(
    name: "StacklyClient",
    platforms: [.macOS("27.0")],
    products: [
        .library(name: "StacklyClient", targets: ["StacklyClient"])
    ],
    targets: [
        // No MainActor default isolation: socket IO runs off the main actor.
        .target(name: "StacklyClient"),
        .testTarget(name: "StacklyClientTests", dependencies: ["StacklyClient"]),
    ]
)
