#![allow(dead_code)]
use std::sync::Arc;
// Using, d3d
use crate::math::Vector2;
use crate::time::{Time, TimeTrait};
// Using
use pixels::{Error, Pixels, SurfaceTexture};
use readonly;
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
    error::EventLoopError
};

pub struct DoomSurface {
    pub size: PhysicalSize<u32>,
    pub pixels: Pixels,
}

#[readonly::make]
pub struct DoomLoopState<C, T: TimeTrait, W> {
    pub context: C,
    #[readonly]
    pub(super) updates_per_second: u32,
    #[readonly]
    pub(super) max_frame_time: f64,
    #[readonly]
    pub(super) exit_next_iteration: bool,
    #[readonly]
    pub(super) window: W,
    #[readonly]
    pub(super) window_occluded: bool,
    #[readonly]
    pub(super) fixed_time_step: f64,
    #[readonly]
    pub(super) number_of_updates: u32,
    #[readonly]
    pub(super) number_of_renders: u32,
    #[readonly]
    pub(super) last_frame_time: f64,
    #[readonly]
    pub(super) running_time: f64,
    #[readonly]
    pub(super) accumulated_time: f64,
    #[readonly]
    pub(super) blending_factor: f64,
    #[readonly]
    pub(super) previous_instant: T,
    #[readonly]
    pub(super) current_instant: T,
}

impl DoomSurface {
    pub fn new(size: PhysicalSize<u32>, window: &Window) -> Option<Self> {
        let surface = SurfaceTexture::new(size.width, size.height, &window);
        if let Ok(pixels) = Pixels::new(size.width, size.height, surface) {
            Some(DoomSurface { size, pixels })
        } else {
            None
        }
    }

    pub fn clear(&mut self, color: [u8; 4]) {
        let frame = self.pixels.frame_mut();
        let size = (frame.len() / color.len()) as usize;
        frame.copy_from_slice(&color.repeat(size));
    }

    pub fn swap(&mut self) -> Result<(), Error> {
        self.pixels.render()
    }

    pub fn draw(&mut self, position: &Vector2<usize>, color: &[u8]) {
        let size = self.pixels.texture().size();
        let channels = self.pixels.texture().format().block_size(None).unwrap() as usize;
        if position.x >= size.width as usize || position.y >= size.height as usize {
            return;
        }
        let frame = self.pixels.frame_mut();
        let row_size = (size.width as usize) * channels; // 4 colors per byte
        let offset: usize =
            (size.height as usize - position.y - 1) * row_size + position.x * channels;
        let mut ptr = frame.as_mut_ptr();
        unsafe {
            ptr = ptr.add(offset);
            for channel in color.iter() {
                (*ptr) = *channel;
                ptr = ptr.add(1);
            }
        }
    }

    pub fn draw_line(&mut self, from: &Vector2<i32>, to: &Vector2<i32>, color: &[u8]){
        let dx = (to.x - from.x).abs();
        let dy = (to.y - from.y).abs();
    
        let step_x = if to.x > from.x { 1 } else { -1 };
        let step_y = if to.y > from.y { 1 } else { -1 };
    
        let mut x = from.x;
        let mut y = from.y;
    
        let mut err = if dx > dy { dx / 2 } else { -dy / 2 };
    
        while x != to.x || y != to.y {
            self.draw(&Vector2::new(x as usize, y as usize), color);
    
            let err2 = err;
    
            if err2 > -dx {
                err -= dy;
                x += step_x;
            }
    
            if err2 < dy {
                err += dx;
                y += step_y;
            }
        }
    }

    pub fn draw_box(&mut self, from: &Vector2<i32>, to: &Vector2<i32>, color: &[u8]){
        self.draw_line(&from, &Vector2::new(from.x, to.y), &color);
        self.draw_line(&from, &Vector2::new(to.x, from.y), &color);
        self.draw_line(&Vector2::new(from.x, to.y), &to, &color);
        self.draw_line(&Vector2::new(to.x, from.y), &to, &color);
    }

}

impl<C, T: TimeTrait, W> DoomLoopState<C, T, W> {
    pub fn new(context: C, updates_per_second: u32, max_frame_time: f64, window: W) -> Self {
        Self {
            context,
            updates_per_second,
            max_frame_time,
            window,
            window_occluded: false,
            exit_next_iteration: false,

            fixed_time_step: 1.0 / updates_per_second as f64,
            number_of_updates: 0,
            number_of_renders: 0,
            running_time: 0.0,
            accumulated_time: 0.0,
            blending_factor: 0.0,
            previous_instant: T::now(),
            current_instant: T::now(),
            last_frame_time: 0.0,
        }
    }

    pub fn next_frame<U, R>(&mut self, mut update: U, mut render: R) -> bool
    where
        U: FnMut(&mut DoomLoopState<C, T, W>),
        R: FnMut(&mut DoomLoopState<C, T, W>),
    {
        let g = self;

        if g.exit_next_iteration {
            return false;
        }

        g.current_instant = T::now();

        let mut elapsed = g.current_instant.sub(&g.previous_instant);
        if elapsed > g.max_frame_time {
            elapsed = g.max_frame_time;
        }

        g.last_frame_time = elapsed;
        g.running_time += elapsed;
        g.accumulated_time += elapsed;

        while g.accumulated_time >= g.fixed_time_step {
            update(g);

            g.accumulated_time -= g.fixed_time_step;
            g.number_of_updates += 1;
        }

        g.blending_factor = g.accumulated_time / g.fixed_time_step;

        if g.window_occluded && T::supports_sleep() {
            T::sleep(g.fixed_time_step);
        } else {
            render(g);
            g.number_of_renders += 1;
        }

        g.previous_instant = g.current_instant;

        true
    }

    pub fn re_accumulate(&mut self) {
        let g = self;

        g.current_instant = T::now();

        let prev_elapsed = g.last_frame_time;
        let new_elapsed = g.current_instant.sub(&g.previous_instant);

        let delta = new_elapsed - prev_elapsed;

        // We don't update g.last_frame_time since this additional time in the
        // render function is considered part of the current frame.

        g.running_time += delta;
        g.accumulated_time += delta;

        g.blending_factor = g.accumulated_time / g.fixed_time_step;
    }

    pub fn exit(&mut self) {
        self.exit_next_iteration = true;
    }

    pub fn set_updates_per_second(&mut self, new_updates_per_second: u32) {
        self.updates_per_second = new_updates_per_second;
        self.fixed_time_step = 1.0 / new_updates_per_second as f64;
    }
}

pub fn doom_window<T: 'static>(
    title: String,
    size: LogicalSize<f64>,
    events: &EventLoop<T>,
) -> Option<Arc<Window>> {
    if let Ok(window) = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(size.clone())
        .with_min_inner_size(size.clone())
        .build(&events)
    {
        Some(Arc::new(window))
    } else {
        None
    }
}

pub fn doom_loop<C, U, R, H, T>(
    event_loop: EventLoop<T>,
    window: Arc<Window>,
    context: C,
    updates_per_second: u32,
    max_frame_time: f64,
    mut update: U,
    mut render: R,
    mut handler: H,
) -> Result<(), EventLoopError>
where
    C: 'static,
    U: FnMut(&mut DoomLoopState<C, Time, Arc<Window>>) + 'static,
    R: FnMut(&mut DoomLoopState<C, Time, Arc<Window>>) + 'static,
    H: FnMut(&mut DoomLoopState<C, Time, Arc<Window>>, &Event<T>) + 'static,
    T: 'static,
{
    let mut doom_loop_state = DoomLoopState::new(context, updates_per_second, max_frame_time, window);
    event_loop.run(move |event: Event<T>, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        // Forward events to existing handlers.
        handler(&mut doom_loop_state, &event);

        match event {
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                if !doom_loop_state.next_frame(&mut update, &mut render) {
                    elwt.exit();
                }
            }
            Event::AboutToWait => {
                doom_loop_state.window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::Occluded(occluded),
                ..
            } => {
                doom_loop_state.window_occluded = occluded;
            }
            _ => {}
        }
    })
}
