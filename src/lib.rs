#![allow(non_snake_case)]

mod bindings_embree;

use std::ptr::null;
use std::slice;
use bindings_embree::*;


// Constants
pub const INVALID_GEOMETRY_ID: u32 = <u32>::MAX;

// Enums
pub type Format = RTCFormat;
pub type BuildQuality = RTCBuildQuality;
pub type DeviceProperty = RTCDeviceProperty;
pub type Error = RTCError;
pub type BufferType = RTCBufferType;
pub type GeometryType = RTCGeometryType;
pub type SubdivisionMode = RTCSubdivisionMode;

// Objects
pub type Device = RTCDevice;
pub type Scene = RTCScene;
pub type Geometry = RTCGeometry;
pub type Ray = RTCRay;
pub type Hit = RTCHit;
pub type RayHit = RTCRayHit;

// API
pub fn CreateDevice() -> RTCDevice {
    let dev;
    unsafe { dev = rtcNewDevice(null()) };
    dev
}

pub fn CreateScene(device: &RTCDevice) -> RTCScene {
    unsafe { rtcNewScene(device.clone()) }
}

pub fn CreateTriangleGeometry(
    device: &RTCDevice,
    scene: &RTCScene,
    vertices: &[(f32, f32, f32)],
    indices: &[(i32, i32, i32)],
) {
    unsafe {
        let geom = rtcNewGeometry(device.clone(), RTCGeometryType::TRIANGLE);

        let vertexBuffPtr = rtcSetNewGeometryBuffer(
            geom,
            RTCBufferType::VERTEX,
            0,
            RTCFormat::FLOAT3,
            3 * size_of::<f32>(),
            vertices.len(),
        );

        if vertexBuffPtr.is_null() {
            panic!("Could not create vertex buffer");
        }

        let indexBuffPtr = rtcSetNewGeometryBuffer(
            geom,
            RTCBufferType::INDEX,
            0,
            RTCFormat::UINT3,
            3 * size_of::<u32>(),
            indices.len(),
        );

        if indexBuffPtr.is_null() {
            panic!("Could not create index buffer");
        }

        let vertexBuff = slice::from_raw_parts_mut(vertexBuffPtr as *mut f32, 3 * vertices.len());
        let indexBuff = slice::from_raw_parts_mut(indexBuffPtr as *mut i32, 3 * indices.len());

        // Copy vertices into buffer
        for (i, v) in vertices.iter().enumerate() {
            vertexBuff[3 * i] = v.0;
            vertexBuff[3 * i + 1] = v.1;
            vertexBuff[3 * i + 2] = v.2;
        }

        // Copy indices into buffer
        for (i, idx) in indices.iter().enumerate() {
            indexBuff[3 * i] = idx.0;
            indexBuff[3 * i + 1] = idx.1;
            indexBuff[3 * i + 2] = idx.2;
        }

        rtcCommitGeometry(geom);
        rtcAttachGeometry(scene.clone(), geom);
        rtcReleaseGeometry(geom);
    }
}

pub fn CommitScene(scene: &RTCScene) {
    unsafe { rtcCommitScene(scene.clone()); };
}

pub fn CastRay(scene: &RTCScene, ray: (f32, f32, f32, f32, f32, f32)) -> Option<RayHit> {
    let mut rayhit = RayHit {
        ray: Ray {
            org_x: ray.0,
            org_y: ray.1,
            org_z: ray.2,
            dir_x: ray.3,
            dir_y: ray.4,
            dir_z: ray.5,
            time: 0.0,
            tnear: 0.0,
            tfar: f32::MAX,
            mask: u32::MAX,
            id: 0,
            flags: 0,
        },
        hit: Hit {
            Ng_x: 0.0,
            Ng_y: 0.0,
            Ng_z: 0.0,
            u: 0.0,
            v: 0.0,
            primID: 0,
            geomID: u32::MAX,
            instID: [u32::MAX],
            instPrimID: [u32::MAX],
        },
    };

    unsafe { rtcIntersect1(scene.clone(), &mut rayhit, std::ptr::null_mut()); }

    if (rayhit.hit.geomID != INVALID_GEOMETRY_ID) {
        Some(rayhit)
    } else {
        None
    }
}