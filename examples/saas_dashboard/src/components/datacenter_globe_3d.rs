//! Interactive 3D Datacenter Topology Visualizer for Strata SaaS Dashboard.
//! Illustrates the Unified 3D API in enterprise analytics: high-level cluster node boxes orbiting a low-level custom shader globe.

use strata::prelude::*;

#[component]
pub fn DatacenterGlobe3D() -> View {
    let rotation = use_signal(0.2);

    rsx! {
        <div class="p-6 rounded-2xl bg-neutral-900/60 border border-neutral-800 shadow-xl flex flex-col gap-4">
            <div class="flex items-center justify-between">
                <div>
                    <h3 class="text-sm font-semibold text-white">{"Global GPU Infrastructure 3D Mesh"}</h3>
                    <p class="text-xs text-neutral-400">{"Live spatial node topology across US, EU, and APAC clusters"}</p>
                </div>
                <div class="flex items-center gap-2">
                    <span class="w-2 h-2 rounded-full bg-emerald-400 animate-ping"></span>
                    <span class="text-xs font-mono text-neutral-300">{"OpenGL / Vulkan Active"}</span>
                </div>
            </div>

            <Canvas3D class="w-full h-64 rounded-xl border border-neutral-800 bg-neutral-950/60 overflow-hidden relative">
                <PerspectiveCamera fov={45.0} position={[0.0, 4.0, 8.0]} target={[0.0, 0.0, 0.0]} />
                <AmbientLight color="#ffffff" intensity={0.3} />
                <DirectionalLight position={[5.0, 8.0, 5.0]} intensity={1.5} cast_shadow={true} />

                {/* Central Custom Shader Globe */}
                <Mesh position={[0.0, 0.0, 0.0]} rotation={[0.0, rotation.get(), 0.0]}>
                    <SphereGeometry radius={1.8} width_segments={48} height_segments={48} />
                    <ShaderMaterial
                        glsl_fragment="
                            precision mediump float;
                            varying vec3 v_normal;
                            void main() {
                                float rim = 1.0 - max(dot(v_normal, vec3(0.0, 0.0, 1.0)), 0.0);
                                vec3 glow = vec3(0.38, 0.40, 0.95) * pow(rim, 2.0);
                                gl_FragColor = vec4(glow, 0.9);
                            }
                        "
                    />
                </Mesh>

                {/* Orbiting High-Level PBR GPU Cluster Node 1: US East */}
                <Mesh position={[2.8, 0.5, 0.0]}>
                    <BoxGeometry width={0.4} height={0.4} depth={0.4} />
                    <StandardMaterial color="#10b981" roughness={0.1} metalness={0.9} />
                </Mesh>

                {/* Orbiting High-Level PBR GPU Cluster Node 2: EU Central */}
                <Mesh position={[-2.4, 0.8, 1.2]}>
                    <BoxGeometry width={0.4} height={0.4} depth={0.4} />
                    <StandardMaterial color="#6366f1" roughness={0.1} metalness={0.9} />
                </Mesh>

                {/* Orbiting High-Level PBR GPU Cluster Node 3: APAC Tokyo */}
                <Mesh position={[0.4, -0.6, 2.7]}>
                    <BoxGeometry width={0.4} height={0.4} depth={0.4} />
                    <StandardMaterial color="#f59e0b" roughness={0.1} metalness={0.9} />
                </Mesh>
            </Canvas3D>
        </div>
    }
}
