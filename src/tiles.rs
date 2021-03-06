use std::rc::Rc;
use crate::texture::Texture;
use crate::screen::Screen;
use crate::types::{Rect, Vec2i,I32};

pub const TILE_SZ: usize = 16;
/// A graphical tile, we'll implement Copy since it's tiny

#[derive(Clone,Copy)]
pub struct Tile {
    pub solid: bool, // ... any extra data like collision flags or other properties
}

/// A set of tiles used in multiple Tilemaps
pub struct Tileset {
    // Tile size is a constant, so we can find the tile in the texture using math
    // (assuming the texture is a grid of tiles).
    pub tiles: Vec<Tile>,
    texture: Rc<Texture>,
    // In this design, each tileset is a distinct image.
    // Maybe not always the best choice if there aren't many tiles in a tileset!
}

impl Tileset {
    /// Create a new tileset
    pub fn new(tiles: Vec<Tile>, texture: &Rc<Texture>) -> Self {
        Self {
            tiles,
            texture: Rc::clone(texture),
        }
    }
    /// Get the frame rect for a tile ID
    fn get_rect(&self, id: TileID) -> Rect {
        let idx = id.0;
        let (w, _h) = self.texture.size();
        let tw = w / TILE_SZ;
        let row = idx / tw;
        let col = idx - (row * tw);
        Rect {
            x: col as i32 * TILE_SZ as i32,
            y: row as i32 * TILE_SZ as i32,
            w: TILE_SZ as u16,
            h: TILE_SZ as u16,
        }
    }
    /// Does this tileset have a tile for `id`?
    fn contains(&self, id: TileID) -> bool {
        id.0 < self.tiles.len()
    }
}
/// Indices into a Tileset
#[derive(Clone,Copy,PartialEq,Eq)]
pub struct TileID(usize);

/// Grab a tile with a given ID
impl std::ops::Index<TileID> for Tileset {
    type Output = Tile;
    fn index(&self, id: TileID) -> &Self::Output {
        &self.tiles[id.0]
    }
}

/// An actual tilemap
pub struct Tilemap {
    /// Where the tilemap is in space, use your favorite number type here
    pub position: Vec2i,
    /// How big it is
    dims: (usize,usize),
    /// Which tileset is used for this tilemap
    tileset: Rc<Tileset>,
    /// A row-major grid of tile IDs in tileset
    map: Vec<TileID>,
}
impl Tilemap {
    // ...
    /// Draws self onto `Screen`
    pub fn draw(&self, screen: &mut Screen) {
        let Rect { x: sx, y: sy, w: sw, h: sh, } = screen.bounds();
        let left = ((sx - self.position.0) / TILE_SZ as i32)
            .max(0)
            .min(self.dims.0 as i32 - 1) as usize;
        let right = ((sx + (sw as i32) - self.position.0) / TILE_SZ as i32)
            .max(0)
            .min(self.dims.0 as i32 - 1) as usize;
        let top = ((sy - self.position.1) / TILE_SZ as i32)
            .max(0)
            .min(self.dims.1 as i32 - 1) as usize;
        let bot = ((sy + (sh as i32) - self.position.1) / TILE_SZ as i32)
            .max(0)
            .min(self.dims.1 as i32 - 1) as usize;
        for (y, row) in (top..=bot).zip(
            self.map[(top * self.dims.0)..((bot + 1) * self.dims.0)]
                .chunks_exact(self.dims.0),
        ) {
            let ypx = (y * TILE_SZ) as i32 + self.position.1;
            for (x, id) in (left..=right).zip(row[left..=right].iter()) {
                let xpx = (x * TILE_SZ) as i32 + self.position.0;
                let frame = self.tileset.get_rect(*id);
                screen.bitblt(&self.tileset.texture, frame, Vec2i(xpx, ypx));
            }
        }
    }
    pub fn new(
        position: Vec2i,
        dims: (usize, usize),
        tileset: &Rc<Tileset>,
        map: Vec<usize>,
    ) -> Self {
        assert_eq!(dims.0 * dims.1, map.len(), "Tilemap is the wrong size!");
        assert!(
            map.iter().all(|tid| tileset.contains(TileID(*tid))),
            "Tilemap refers to nonexistent tiles"
        );
        Self {
            position,
            dims,
            tileset: Rc::clone(tileset),
            map: map.into_iter().map(TileID).collect(),
        }
    }

    pub fn tile_id_at(&self, Vec2i(x, y): Vec2i) -> TileID {
        // Translate into map coordinates
        let x = (x - self.position.0) / TILE_SZ as i32;
        let y = (y - self.position.1) / TILE_SZ as i32;
        assert!(
            x >= 0 && x < self.dims.0 as i32,
            "Tile X coordinate {} out of bounds {}",
            x,
            self.dims.0
        );
        assert!(
            y >= 0 && y < self.dims.1 as i32,
            "Tile Y coordinate {} out of bounds {}",
            y,
            self.dims.1
        );
        self.map[y as usize * self.dims.0 + x as usize]
    }
    pub fn size(&self) -> (usize, usize) {
        self.dims
    }
    pub fn tile_at(&self, posn: Vec2i) -> Tile {
        self.tileset[self.tile_id_at(posn)]
    }
}
