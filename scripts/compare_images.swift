import AppKit
import CoreGraphics
import Foundation

// ============================================================================
// macOS Native Visual Parity & Fidelity Comparison Engine
// Computes byte identity, Mean Squared Error (MSE), and Structural Similarity
// (SSIM) between GPUI and Telegram Desktop reference snapshots.
// ============================================================================

func getGrayscalePixels(from url: URL) -> (pixels: [Double], width: Int, height: Int)? {
    guard let img = NSImage(contentsOf: url),
          let cgImg = img.cgImage(forProposedRect: nil, context: nil, hints: nil) else {
        return nil
    }
    let w = cgImg.width
    let h = cgImg.height
    let colorSpace = CGColorSpaceCreateDeviceGray()
    var rawData = [UInt8](repeating: 0, count: w * h)

    guard let ctx = CGContext(
        data: &rawData,
        width: w,
        height: h,
        bitsPerComponent: 8,
        bytesPerRow: w,
        space: colorSpace,
        bitmapInfo: CGImageAlphaInfo.none.rawValue
    ) else {
        return nil
    }

    ctx.draw(cgImg, in: CGRect(x: 0, y: 0, width: w, height: h))
    let doublePixels = rawData.map { Double($0) }
    return (doublePixels, w, h)
}

func computeMetrics(url1: URL, url2: URL) -> [String: Any]? {
    guard let img1 = getGrayscalePixels(from: url1),
          let img2 = getGrayscalePixels(from: url2),
          img1.width == img2.width, img1.height == img2.height else {
        return nil
    }

    let w = img1.width
    let h = img1.height
    let n = Double(w * h)

    var sumSqDiff = 0.0
    var mean1 = 0.0
    var mean2 = 0.0
    var isIdentical = true

    for i in 0..<img1.pixels.count {
        let p1 = img1.pixels[i]
        let p2 = img2.pixels[i]
        let diff = p1 - p2
        if diff != 0.0 {
            isIdentical = false
        }
        sumSqDiff += diff * diff
        mean1 += p1
        mean2 += p2
    }

    let mse = sumSqDiff / n
    mean1 /= n
    mean2 /= n

    var var1 = 0.0
    var var2 = 0.0
    var cov = 0.0

    for i in 0..<img1.pixels.count {
        let d1 = img1.pixels[i] - mean1
        let d2 = img2.pixels[i] - mean2
        var1 += d1 * d1
        var2 += d2 * d2
        cov += d1 * d2
    }

    var1 /= n
    var2 /= n
    cov /= n

    let k1 = 0.01
    let k2 = 0.03
    let L = 255.0
    let c1 = (k1 * L) * (k1 * L)
    let c2 = (k2 * L) * (k2 * L)

    let ssim = ((2.0 * mean1 * mean2 + c1) * (2.0 * cov + c2)) /
               ((mean1 * mean1 + mean2 * mean2 + c1) * (var1 + var2 + c2))

    let psnr = mse > 0 ? (10.0 * log10((L * L) / mse)) : 99.9
    let parityPercent = max(0.0, min(100.0, ssim * 100.0))

    return [
        "img1": url1.lastPathComponent,
        "img2": url2.lastPathComponent,
        "width": w,
        "height": h,
        "is_identical": isIdentical,
        "mse": Double(round(mse * 1000) / 1000),
        "psnr_db": Double(round(psnr * 100) / 100),
        "ssim": Double(round(ssim * 10000) / 10000),
        "parity_percent": Double(round(parityPercent * 10) / 10)
    ]
}

let args = CommandLine.arguments
if args.count >= 3 {
    let u1 = URL(fileURLWithPath: args[1])
    let u2 = URL(fileURLWithPath: args[2])
    if let result = computeMetrics(url1: u1, url2: u2),
       let data = try? JSONSerialization.data(withJSONObject: result, options: [.prettyPrinted]),
       let jsonStr = String(data: data, encoding: .utf8) {
        print(jsonStr)
    } else {
        print("{\"error\": \"Failed to compute visual metrics\"}")
        exit(1)
    }
} else {
    print("Usage: swift compare_images.swift <image1.png> <image2.png>")
    exit(1)
}
