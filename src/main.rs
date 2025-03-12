use std::collections::VecDeque;

struct Page {
    number: i32,
    reference: bool,
    last_used: usize,
}

fn wsclock_page_replacement(
    pages: Vec<i32>,
    frame_size: usize,
    tau: usize,
) -> usize {
    let mut frames: VecDeque<Page> = VecDeque::new();
    let mut clock_hand = 0; // Pointer to the clock hand
    let mut page_faults = 0;
    let mut time = 0;

    for &page in &pages {
        time += 1; // Increment the "time" on each page access

        // Check if the page is already in frames
        if let Some(existing_page) = frames.iter_mut().find(|p| p.number == page) {
            existing_page.reference = true; // Update the reference bit
            existing_page.last_used = time; // Update the last used time
            continue;
        }

        // Page fault
        page_faults += 1;

        if frames.len() == frame_size {
            // Scan the clock to find a page to replace
            let mut scanned_pages = 0; // Track number of scanned pages to avoid infinite loop
            loop {
                let hand_page = &mut frames[clock_hand];
                if !hand_page.reference {
                    // Check the age of the page
                    if time - hand_page.last_used > tau {
                        // Evict the page
                        frames.remove(clock_hand);
                        // Adjust the clock hand after removal
                        if clock_hand >= frames.len() {
                            clock_hand = 0;
                        }
                        break;
                    }
                } else {
                    // Give the page a second chance and reset its reference bit
                    hand_page.reference = false;
                }

                // Move the clock hand
                clock_hand = (clock_hand + 1) % frames.len();
                scanned_pages += 1;

                // Break to prevent infinite loop if all pages have been scanned
                if scanned_pages == frames.len() {
                    frames.pop_front(); // Evict the oldest page as a fallback
                    clock_hand = 0; // Reset the clock hand
                    break;
                }
            }
        }

        // Insert the new page
        frames.push_back(Page {
            number: page,
            reference: true,
            last_used: time,
        });
    }

    page_faults
}

fn main() {
    let pages = vec![7, 0, 1, 2, 0, 3, 0, 4, 2, 3, 0, 3, 2];
    let frame_size = 3;
    let tau = 4; // Working set window size

    let faults = wsclock_page_replacement(pages.clone(), frame_size, tau);
    println!("Number of page faults: {}", faults);
}
