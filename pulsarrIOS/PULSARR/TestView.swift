//
//  TestView.swift
//  PULSARR
//
//  Created by Ethan Hart on 10/6/24.
//

import SwiftUI


@main
struct AlbumReviewsApp: App {
    var body: some Scene {
        WindowGroup {
            HomePage()
        }
    }
}

struct HomePage: View {
    @Namespace private var namespace
    @State private var show = false
    
    var thumbnail : ThumbnailView
    var expanded: ExpandedView
    
    var body: some View {
        ZStack {
            if !show {
                thumbnailView()
            } else {
                expandedView()
            }
        }
        .onTapGesture {
            if !show {
                withAnimation(.spring(response:0.6, dampingFraction:0.8)){
                    show.toggle()
                }
            }
        }
    }
    
    @ViewBuilder
    private func thumbnailView() -> some View {
        ZStack {
            thumbnail
                .matchedGeometryEffect(id: "vuew", in: namespace)
        }
        .background(
            Color.ui.background
                .matchedGeometryEffect(id: "background", in: namespace)
        )
        .mask {
            RoundedRectangle(cornerRadius: 20,  style: .continuous)
                .matchedGeometryEffect(id: "mask", in: namespace)
        }
        
        Button(action: {
            
        }, label: {
            Image(systemName: "xmark")
                .foregroundColor(.white)
        })
        .padding()
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topTrailing)
        .matchedGeometryEffect(id: "mask", in: namespace)
    }
    
    @ViewBuilder
    private func expandedView() -> some View {
        ZStack {
            expanded
                .matchedGeometryEffect(id: "view", in: namespace)
                .background(
                    Color.ui.card_background
                        .matchedGeometryEffect(id: "background", in: namespace)
                )
                .mask {
                    RoundedRectangle(cornerRadius: 20, style: .continuous)
                        .matchedGeometryEffect(id: "mask", in: namespace)
                }
        }
    }
}

struct ThumbnailView: View, Identifiable {
    var id = UUID()
    @ViewBuilder var content: any View
    
    var body: some View {
        ZStack {
            AnyView(content)
        }
    }
}

struct ExpandedView: View, Identifiable {
    var id = UUID()
    @ViewBuilder var content: any View
    
    var body: some View {
        ZStack {
            AnyView(content)
        }
    }
}


#Preview {
    HomePage()
}
