// swift-tools-version:5.5
import PackageDescription

let package = Package(
    name: "TelephonyPlugin",
    platforms: [
        .iOS(.v13)
    ],
    products: [
        .library(
            name: "TelephonyPlugin",
            targets: ["TelephonyPlugin"]
        )
    ],
    dependencies: [
        .package(name: "Tauri", path: "../.tauri/tauri-api")
    ],
    targets: [
        .target(
            name: "TelephonyPlugin",
            dependencies: [
                .product(name: "Tauri", package: "Tauri")
            ],
            path: "Sources/TelephonyPlugin"
        )
    ]
)
