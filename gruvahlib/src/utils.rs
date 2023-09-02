/*
 * Copyright (c) 2023 Joshua Azimullah
 *
 * This file is part of Gruvah.
 *
 * Gruvah is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 * Gruvah is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with Gruvah. If not, see <https://www.gnu.org/licenses/>.
 */


pub fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 10.0)
}

#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;
    use crate::utils::db_to_linear;

    #[test]
    fn test_0_db() {
        assert_approx_eq!(db_to_linear(0.0), 1.0, 0.005);
    }

    #[test]
    fn test_6_db() {
        assert_approx_eq!(db_to_linear(6.0), 2.0, 0.005);
    }
}
