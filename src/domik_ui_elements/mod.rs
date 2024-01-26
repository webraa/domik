






//  //  //  //  //  //  //  //
//      Basic test element
//  //  //  //  //  //  //  //
pub fn dom_lvl(lvl: i8) -> impl egui::Widget {
    move |ui: &mut egui::Ui| dom_lvl_responder(ui, lvl)
}

fn dom_lvl_responder( ui: &mut egui::Ui, lvl: i8 ) -> egui::Response {
    let clr = lvl2color(lvl);
    let de_oct_lvl = de_oct_lvl( lvl );

    let desired_size = ui.spacing().interact_size.y * 2. * egui::vec2(2.5, 1.);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        // do something
        response.mark_changed();
    }
    //response.widget_info(|| egui::WidgetInfo::selected(typ, selected, label));
    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact_selectable(&response, true );
        let rect = rect.expand(visuals.expansion);
        let _radius = 0.1 * rect.height();
        let points: Vec<egui::Pos2> = match de_oct_lvl {
            -3 | -1 | 2 | 4 | 6 => {
                vec![
                    egui::Pos2::new( rect.left(), rect.bottom() ),
                    egui::Pos2::new( rect.right(), rect.bottom() ),
                    egui::Pos2::new( rect.left()+0.5*rect.width(), rect.top() )
                ]
            },
            -4 | -2 | 1 | 3 | 5 => {
                vec![
                    egui::Pos2::new( rect.right(), rect.top() ),
                    egui::Pos2::new( rect.left(), rect.top() ),
                    egui::Pos2::new( rect.left()+0.5*rect.width(), rect.bottom() )
                ]
            },
            _ => {
                vec![
                    egui::Pos2::new( rect.left(), rect.bottom() ),
                    egui::Pos2::new( rect.right(), rect.bottom() ),
                    egui::Pos2::new( rect.right(), rect.top() ),
                    egui::Pos2::new( rect.left(), rect.top() )
                ]
            },
        };

        let poly = egui::Shape::convex_polygon(points, clr, visuals.bg_stroke );
        ui.painter().add( poly );
    }
    response
}


//  //  //  //  //  //  //  //
//      UTILs
//  //  //  //  //  //  //  //
fn de_oct_lvl( lvl: i8 ) -> i8 {
    let clear_lvl = lvl % 12;
    if clear_lvl < -5 {
        return clear_lvl + 12
    }
    if clear_lvl > 7 {
        return clear_lvl - 12
    }
   clear_lvl
}
fn lvl2color( lvl: i8 ) -> egui::Color32 {
    let de_oct = de_oct_lvl( lvl );
    match de_oct {
         7 |
        -5 => egui::Color32::from_rgb( 173,255, 47 ),
         6 => egui::Color32::from_rgb( 255, 36,  0 ),
         5 => egui::Color32::from_rgb(  46,232,187 ),
         4 => egui::Color32::from_rgb( 255,165,  0 ),
         3 => egui::Color32::from_rgb(   0,  0,255 ),
         2 => egui::Color32::from_rgb( 253,233, 16 ),
         1 => egui::Color32::from_rgb( 127,  0,127 ),
         0 => egui::Color32::from_rgb(  86,127, 24 ),
        -1 => egui::Color32::from_rgb( 204,  0,  0 ),
        -2 => egui::Color32::from_rgb( 128,166,255 ),
        -3 => egui::Color32::from_rgb( 217,204,  0 ),
        -4 => egui::Color32::from_rgb( 127,  0,255 ),
        _ => {
            egui::Color32::from_rgb( 222,222,222 )
        }
    }
}




//  //  //  //  //  //  //  //
//      TESTS
//  //  //  //  //  //  //  //

#[cfg(test)]
mod domik_utils{
    use super::*;

    #[test]
    fn should_not_change() {
        for lvl in -5..=7 {
            //println!("test:lvl:{} -> {}", lvl, de_oct_lvl(lvl) );
            let de_oct = de_oct_lvl(lvl);
            assert!( lvl == de_oct, "should not be changed" );
        }
    }
    #[test]
    fn should_change_oct_up() {
        for lvl in -5..=6 {
            let shifted = lvl - 12;
            let de_oct = de_oct_lvl(shifted);
            //println!("test:lvl:{} -> {}", shifted, de_oct );
            assert!( lvl == de_oct, "should be an octave rised" );
        }
    }
    #[test]
    fn should_change_oct_down() {
        for lvl in -4..=7 {
            let shifted = lvl + 12;
            let de_oct = de_oct_lvl(shifted);
            //println!("test:lvl:{} -> {}", shifted, de_oct );
            assert!( lvl == de_oct, "should be an octave skiped" );
        }
    }
    #[test]
    fn should_change_2oct_up() {
        for lvl in -5..=6 {
            let shifted = lvl - 24;
            let de_oct = de_oct_lvl(shifted);
            //println!("test:lvl:{} -> {}", shifted, de_oct );
            assert!( lvl == de_oct, "should be a two octave rised" );
        }
    }
    #[test]
    fn should_change_2oct_down() {
        for lvl in -4..=7 {
            let shifted = lvl + 24;
            let de_oct = de_oct_lvl(shifted);
            //println!("test:lvl:{} -> {}", shifted, de_oct );
            assert!( lvl == de_oct, "should be a two octave skiped" );
        }
    }
}
