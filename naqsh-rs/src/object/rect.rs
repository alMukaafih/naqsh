pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    /// Creates a new rectangle with the specified coordinates. Note: no range
    /// checking is performed, so the caller must ensure that `left <= right` and
    /// `top <= bottom`.
    ///
    /// `left` is the X coordinate of the left side of the rectangle.
    /// `top` is the Y coordinate of the top of the rectangle.
    /// `right` is the X coordinate of the right side of the rectangle.
    /// `bottom` is the Y coordinate of the bottom of the rectangle.
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left, top, right, bottom
        }
    }

    /// Returns `true` if the rectangle is empty (`left >= right` or `top >= bottom`)
    pub fn is_empty(&self) -> bool {
        self.left >= self.right || self.top >= self.bottom
    }

    /// Returns `true` if the rectangle is valid (`left <= right` and `top <= bottom`).
    pub fn is_valid(&self) -> bool {
        self.left <= self.right && self.top <= self.bottom
    }

    /// Returns the rectangle's width. self does not check for a valid rectangle
    /// (i.e. left <= right) so the result may be negative.
    pub fn width(&self) -> i32 {
        self.right - self.left
    }

    /// Returns the rectangle's height. self does not check for a valid rectangle
    /// (i.e. top <= bottom) so the result may be negative.
    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }

    pub fn center_x(&self) -> i32 {
        (self.left + self.right) >> 1
    }

    pub fn center_y(&self) -> i32 {
        (self.top + self.bottom) >> 1
    }

    pub fn exact_center_x(&self) -> f32 {
        (self.left + self.right) as f32 * 0.5_f32
    }

    pub fn exact_center_y(&self) -> f32 {
        (self.top + self.bottom) as f32 * 0.5_f32
    }

    pub fn set(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.left = left;
        self.top = top;
        self.right = right;
        self.bottom = bottom;
    }

    pub fn offset(&mut self, dx: i32, dy: i32) {
        self.left += dx;
        self.top += dy;
        self.right += dx;
        self.bottom += dy;
    }

    pub fn offset_to(&mut self, new_left: i32, new_top: i32) {
        self.right += new_left - self.left;
        self.bottom += new_top - self.top;
        self.left = new_left;
        self.top = new_top;
    }

    pub fn intersect(&mut self, left: i32, top: i32, right: i32, bottom: i32) -> bool {
        if self.left < right && left < self.right && self.top < bottom && top < self.bottom {
            if self.left < left {self.left = left;}
            if self.top < top {self.top = top;}
            if self.right > right {self.right = right;}
            if self.bottom > bottom {self.bottom = bottom;}
            return true;
        }
        false
    }

    pub fn intersect_unchecked(&mut self, other: Rect) {
        self.left = i32::max(self.left, other.left);
        self.top = i32::max(self.top, other.top);
        self.right = i32::min(self.right, other.right);
        self.bottom = i32::min(self.bottom, other.bottom);
    }

    pub fn intersects(&self, left: i32, top: i32, right: i32, bottom: i32) -> bool {
        self.left < right && left < self.right && self.top < bottom && top < self.bottom
    }

    pub fn scale(&mut self, scale: f32) {
        if scale != 1.0_f32 {
            self.left = (self.left as f32 * scale + 0.5_f32) as i32;
            self.top = (self.top as f32 * scale + 0.5_f32) as i32;
            self.right = (self.right as f32 * scale + 0.5_f32) as i32;
            self.bottom = (self.bottom as f32 * scale + 0.5_f32) as i32;
        }
    }
}


impl Default for Rect {
    /// Create a new empty Rect. All coordinates are initialized to 0.
    fn default() -> Self {
        Self {
            left: Default::default(),
            top: Default::default(),
            right: Default::default(),
            bottom: Default::default()
        }
    }
}