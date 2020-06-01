use crate::typed_array::TypedArray;
use crate::Color;

/// A reference-counted vector of `Color` that uses Godot's pool allocator.
pub type ColorArray = TypedArray<Color>;

godot_test!(
    test_color_array_access {
        let mut arr = ColorArray::new();
        arr.push_ref(&Color::rgb(1.0, 0.0, 0.0));
        arr.push_ref(&Color::rgb(0.0, 1.0, 0.0));
        arr.push_ref(&Color::rgb(0.0, 0.0, 1.0));

        let original_read = {
            let read = arr.read();
            assert_eq!(&[
                Color::rgb(1.0, 0.0, 0.0),
                Color::rgb(0.0, 1.0, 0.0),
                Color::rgb(0.0, 0.0, 1.0),
            ], read.as_slice());
            read.clone()
        };

        let mut cow_arr = arr.new_ref();

        {
            let mut write = cow_arr.write();
            assert_eq!(3, write.len());
            for i in write.as_mut_slice() {
                i.b = 1.0;
            }
        }

        assert_eq!(Color::rgb(1.0, 0.0, 1.0), cow_arr.get(0));
        assert_eq!(Color::rgb(0.0, 1.0, 1.0), cow_arr.get(1));
        assert_eq!(Color::rgb(0.0, 0.0, 1.0), cow_arr.get(2));

        // the write shouldn't have affected the original array
        assert_eq!(&[
            Color::rgb(1.0, 0.0, 0.0),
            Color::rgb(0.0, 1.0, 0.0),
            Color::rgb(0.0, 0.0, 1.0),
        ], original_read.as_slice());
    }
);

godot_test!(
    test_color_array_debug {
        let mut arr = ColorArray::new();
        arr.push_ref(&Color::rgb(1.0, 0.0, 0.0));
        arr.push_ref(&Color::rgb(0.0, 1.0, 0.0));
        arr.push_ref(&Color::rgb(0.0, 0.0, 1.0));

        assert_eq!(format!("{:?}", arr), "[Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }, Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }, Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 }]");
    }
);
