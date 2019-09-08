extern crate user32;
extern crate winapi;

use winapi::{
    shared::{dxgi, dxgiformat, dxgitype, windef::HWND, winerror::HRESULT},
    um::{
        d3d11::{
            D3D11CreateDeviceAndSwapChain, ID3D11Buffer, ID3D11Device, ID3D11DeviceContext,
            ID3D11InputLayout, ID3D11PixelShader, ID3D11RenderTargetView, ID3D11Resource,
            ID3D11SamplerState, ID3D11ShaderResourceView, ID3D11Texture2D, ID3D11VertexShader,
            D3D11_MAPPED_SUBRESOURCE, D3D11_SDK_VERSION, D3D11_VIEWPORT, D3D11_TEXTURE2D_DESC,
            D3D11_USAGE_DYNAMIC, D3D11_BIND_SHADER_RESOURCE, D3D11_CPU_ACCESS_WRITE,
            D3D11_SHADER_RESOURCE_VIEW_DESC, D3D11_SHADER_RESOURCE_VIEW_DESC_u
        },
        d3dcommon,
    },
    Interface,
};

use std::ptr::{null, null_mut};

pub struct Graphics {
    width: i32,
    height: i32,
    swap_chain: *mut *mut dxgi::IDXGISwapChain,
    device: *mut *mut ID3D11Device,
    device_context: *mut *mut ID3D11DeviceContext,
    render_target: *mut *mut ID3D11RenderTargetView,
    buffer_texture: *mut *mut ID3D11Texture2D,
    texture_view: *mut *mut ID3D11ShaderResourceView,
    pixel_shader: *mut *mut ID3D11PixelShader,
    vertex_shader: *mut *mut ID3D11VertexShader,
    vertex_buffer: *mut *mut ID3D11Buffer,
    input_layout: *mut *mut ID3D11InputLayout,
    sampler_state: *mut *mut ID3D11SamplerState,
}

impl Graphics {
    pub fn new() -> Graphics {
        let width = 800;
        let height = 600;
        Graphics {
            width: width,
            height: height,
            swap_chain: null_mut(),
            device: null_mut(),
            device_context: null_mut(),
            render_target: null_mut(),
            buffer_texture: null_mut(),
            texture_view: null_mut(),
            pixel_shader: null_mut(),
            vertex_shader: null_mut(),
            vertex_buffer: null_mut(),
            input_layout: null_mut(),
            sampler_state: null_mut(),
        }
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn assign_swap_chain(&self, window_handle: HWND) {
        let swap_chain_desc = dxgi::DXGI_SWAP_CHAIN_DESC {
            BufferCount: 1,
            // Formats how the pixels in the buffers are set up in memory
            BufferDesc: dxgitype::DXGI_MODE_DESC {
                Width: self.width as u32,
                Height: self.height as u32,
                Format: dxgiformat::DXGI_FORMAT_B8G8R8A8_UNORM,
                RefreshRate: dxgitype::DXGI_RATIONAL {
                    Numerator: 1,
                    Denominator: 60,
                },
                ScanlineOrdering: 0,
                Scaling: 0,
            },
            BufferUsage: dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT,
            OutputWindow: window_handle,
            SampleDesc: dxgitype::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Windowed: 1,
            Flags: 0,
            SwapEffect: 0,
        };
        let mut hr: HRESULT;
        unsafe {
            hr = D3D11CreateDeviceAndSwapChain(
                null_mut(),                          // *pAdapter
                d3dcommon::D3D_DRIVER_TYPE_HARDWARE, // DriverType
                null_mut(),                          //HMODULE
                0,                                   //FLAGS
                null_mut(),                          // *FeatureLevels
                0,                                   // FeatureLEvels
                D3D11_SDK_VERSION,
                &swap_chain_desc,
                self.swap_chain,
                self.device,
                null_mut(),
                self.device_context,
            );
            handle_HR("Error on D3D11CreateDeviceAndSwapChain.", hr);

            let back_buffer: *mut *mut ID3D11Resource = null_mut();
            ID3D11Buffer::uuidof();
            hr = (*(*self.swap_chain)).GetBuffer(
                0, 
                &ID3D11Texture2D::uuidof(),
                back_buffer as *mut *mut _ as *mut *mut std::ffi::c_void
            );
            handle_HR("Error on grabbing back buffer.", hr);

            hr = (*(*self.device)).CreateRenderTargetView(
                *back_buffer,
                null(),
                self.render_target
            );

            handle_HR("Error on Creating Render Target.", hr);

            (*(*self.device_context)).OMSetRenderTargets(
                1,
                self.render_target,
                null_mut()
            );

            let viewport = D3D11_VIEWPORT {
                Width: self.width as f32,
                Height: self.height as f32,
                MinDepth: 0 as f32,
                MaxDepth: 1 as f32,
                TopLeftX: 0 as f32,
                TopLeftY: 0 as f32
            };

            (*(*self.device_context)).RSSetViewports(1, &viewport);

            let texture_desc = D3D11_TEXTURE2D_DESC {
                Width: self.width as u32,
                Height: self.height as u32,
                MipLevels: 1,
                ArraySize: 1,
                Format: dxgiformat::DXGI_FORMAT_B8G8R8A8_UNORM,
                SampleDesc: dxgitype::DXGI_SAMPLE_DESC{
                    Count: 1,
                    Quality: 0,
                },
                Usage: D3D11_USAGE_DYNAMIC,
                BindFlags: D3D11_BIND_SHADER_RESOURCE,
                CPUAccessFlags: D3D11_CPU_ACCESS_WRITE,
                MiscFlags: 0
            };
            hr = (*(*self.device)).CreateTexture2D(
                &texture_desc,
                null_mut(),
                self.buffer_texture
            );

            handle_HR("Error on Creating 2D texture.", hr);

            let u: D3D11_SHADER_RESOURCE_VIEW_DESC_u = std::mem::zeroed();
            u.Texture2D();
            let shader_resource_desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
                Format: texture_desc.Format,
                ViewDimension: d3dcommon::D3D11_SRV_DIMENSION_TEXTURE2D,
                u: u,
            };
        }
    }
}

pub fn handle_HR(string: &str, hr: HRESULT) {
    if hr != 0 {
        panic!("{}\nERR CODE: 0x{:x}", string, hr);
    }
}