import {
  AdditiveBlending,
  BufferGeometry,
  Color,
  Float32BufferAttribute,
  PerspectiveCamera,
  Points,
  PointsMaterial,
  Scene,
  WebGPURenderer,
} from 'three/webgpu';
import { WebGLRenderer } from 'three';

import { bootClientComponent, prefersReducedMotion, type ClientCleanup } from '../resuma-client';

const COUNT = 3200;
const PURPLE = new Color('#712cf9');
const BLUE = new Color('#0550ae');
const SOFT = new Color('#e9d5ff');

function initHeroParticles(root: HTMLElement): ClientCleanup | void {
  if (prefersReducedMotion()) return;

  const canvas = document.createElement('canvas');
  canvas.className = 'hero-particles-canvas';
  canvas.setAttribute('aria-hidden', 'true');
  root.appendChild(canvas);

  const positions = new Float32Array(COUNT * 3);
  const colors = new Float32Array(COUNT * 3);
  const seeds = new Float32Array(COUNT * 3);

  for (let i = 0; i < COUNT; i += 1) {
    const i3 = i * 3;
    positions[i3] = (Math.random() - 0.5) * 14;
    positions[i3 + 1] = (Math.random() - 0.5) * 8;
    positions[i3 + 2] = (Math.random() - 0.5) * 6;

    const mix = Math.random();
    const c = PURPLE.clone()
      .lerp(BLUE, mix * 0.85)
      .lerp(SOFT, Math.random() * 0.25);
    colors[i3] = c.r;
    colors[i3 + 1] = c.g;
    colors[i3 + 2] = c.b;

    seeds[i3] = Math.random() * Math.PI * 2;
    seeds[i3 + 1] = 0.4 + Math.random() * 1.2;
    seeds[i3 + 2] = 0.15 + Math.random() * 0.55;
  }

  const geometry = new BufferGeometry();
  geometry.setAttribute('position', new Float32BufferAttribute(positions, 3));
  geometry.setAttribute('color', new Float32BufferAttribute(colors, 3));

  const material = new PointsMaterial({
    size: 0.045,
    vertexColors: true,
    transparent: true,
    opacity: 0.72,
    blending: AdditiveBlending,
    depthWrite: false,
    sizeAttenuation: true,
  });

  const points = new Points(geometry, material);
  const scene = new Scene();
  scene.add(points);

  const camera = new PerspectiveCamera(55, 1, 0.1, 100);
  camera.position.z = 5.5;

  let renderer: WebGPURenderer | WebGLRenderer | undefined;
  let pointerX = 0;
  let pointerY = 0;
  let raf = 0;
  let visible = true;

  const resize = () => {
    if (!renderer) return;
    const { clientWidth: w, clientHeight: h } = root;
    if (!w || !h) return;
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
  };

  const onMove = (event: PointerEvent) => {
    const rect = root.getBoundingClientRect();
    pointerX = ((event.clientX - rect.left) / rect.width - 0.5) * 0.35;
    pointerY = ((event.clientY - rect.top) / rect.height - 0.5) * 0.2;
  };

  const animate = (time: number) => {
    if (!renderer) return;
    if (visible) {
      const t = time * 0.001;
      const pos = geometry.attributes.position.array as Float32Array;

      for (let i = 0; i < COUNT; i += 1) {
        const i3 = i * 3;
        const sx = seeds[i3];
        const sy = seeds[i3 + 1];
        const sz = seeds[i3 + 2];
        pos[i3] += Math.sin(t * sy + sx) * 0.0009;
        pos[i3 + 1] += Math.cos(t * sz + sx * 1.3) * 0.0007;
        pos[i3 + 2] += Math.sin(t * 0.6 + sx) * 0.0005;
      }
      geometry.attributes.position.needsUpdate = true;

      points.rotation.y = t * 0.04 + pointerX;
      points.rotation.x = pointerY * 0.35;
      camera.position.x = pointerX * 0.6;
      camera.position.y = -pointerY * 0.4;
      camera.lookAt(0, 0, 0);

      renderer.render(scene, camera);
    }
    raf = requestAnimationFrame(animate);
  };

  const observer = new IntersectionObserver(
    (entries) => {
      visible = entries.some((entry) => entry.isIntersecting);
    },
    { root: null, threshold: 0.05 },
  );
  observer.observe(root);

  window.addEventListener('pointermove', onMove, { passive: true });
  window.addEventListener('resize', resize);

  const start = async () => {
    try {
      renderer = new WebGPURenderer({
        canvas,
        alpha: true,
        antialias: true,
        powerPreference: 'high-performance',
      });
      renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
      await renderer.init();
    } catch {
      renderer = new WebGLRenderer({
        canvas,
        alpha: true,
        antialias: true,
        powerPreference: 'high-performance',
      });
      renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    }

    resize();
    raf = requestAnimationFrame(animate);
  };

  start().catch(() => {
    root.remove();
  });

  return () => {
    cancelAnimationFrame(raf);
    observer.disconnect();
    window.removeEventListener('pointermove', onMove);
    window.removeEventListener('resize', resize);
    geometry.dispose();
    material.dispose();
    renderer?.dispose?.();
  };
}

bootClientComponent('hero-particles', initHeroParticles);
