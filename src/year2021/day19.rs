use std::cmp::max;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Add, Sub};

use ahash::{HashMap, HashMapExt};
use bstr::ByteSlice;
use derive_new::new;
use itertools::Itertools;

use parse_yolo_derive::ParseYolo;

use crate::array::Vector3d;
use crate::input::{InputData, U8IteratorExtras, U8SliceExtras};

const MIN_OVERLAP: usize = 12;

pub fn part_1(input: &InputData) -> usize {
    let processed_scanners = process_scanners(input);

    let mut num_beacons = processed_scanners[0].report.beacons.len();

    for i in 1..processed_scanners.len() {
        num_beacons += processed_scanners[i].report.beacons.iter()
            .filter(|beacon| !processed_scanners.iter().take(i).any(|scanner| scanner.contains(beacon)))
            .count();
    }

    num_beacons
}

pub fn part_2(input: &InputData) -> u32 {
    let processed_scanners = process_scanners(input);
    let mut max_distance = 0;
    for i in 0..processed_scanners.len() {
        for j in 0..i {
            max_distance = max(max_distance, processed_scanners[i].scanner_position.manhattan_distance(&processed_scanners[j].scanner_position));
        }
    }
    max_distance
}

fn process_scanners(input: &InputData) -> Vec<ProcessedScanner> {
    let mut scanner_reports = input.lines()
        .map_chunks(|mut chunk| {
            chunk.next();
            let beacons = chunk.map(|line| line.stream().parse_yolo::<BeaconPosition>()).map(|beacon| beacon.as_vector()).collect_vec();
            let mut graph = HashMap::with_capacity(beacons.len() * (beacons.len() - 1) / 2);
            for i in 0..beacons.len() {
                for j in 0..i {
                    graph.insert(DistanceInvariant::new(&beacons[i], &beacons[j]), (i, j));
                }
            }

            ScannerReport::new(beacons, graph)
        })
        .collect_vec();

    let mut processed_scanners = Vec::with_capacity(scanner_reports.len());
    processed_scanners.push(ProcessedScanner::new(Vector3d::new(0, 0, 0), scanner_reports.swap_remove(0)));

    // let mut num_beacons = processed_scanners[0].report.beacons.len();

    while !scanner_reports.is_empty() {
        'outer: for i in 0..scanner_reports.len() {
            for j in 0..processed_scanners.len() {
                if let Some((relative_scanner_position, transformation)) = find_match(&processed_scanners[j], &scanner_reports[i]) {
                    let transformed_beacon_positions = scanner_reports[i].beacons.iter()
                        .map(|beacon| &relative_scanner_position + &transformation.transform(beacon))
                        .collect_vec();

                    // num_beacons += transformed_beacon_positions.iter()
                    //     .filter(|beacon| !processed_scanners.iter().any(|scanner| scanner.contains(beacon)))
                    //     .count();

                    let original_report = scanner_reports.swap_remove(i);
                    processed_scanners.push(
                        ProcessedScanner::new(
                            relative_scanner_position,
                            ScannerReport::new(
                                transformed_beacon_positions,
                                original_report.graph,
                            ),
                        )
                    );

                    break 'outer;
                }
            }
        }
    }

    processed_scanners
}

fn find_match(processed_scanner: &ProcessedScanner, scanner_report: &ScannerReport) -> Option<(Vector3d, Vector3dTransformation)> {
    if processed_scanner.intersection(scanner_report).count() >= MIN_OVERLAP * (MIN_OVERLAP - 1) / 2 {
        let (invariant_1, ((beacon_1_1, beacon_1_2), (beacon_2_1, beacon_2_2))) = processed_scanner.intersection(scanner_report)
            .find(|(distance, _)| distance.elements[0] != distance.elements[1] && distance.elements[1] != distance.elements[2] && distance.elements[0] != distance.elements[2])
            .unwrap();
        for (invariant_2, ((beacon_1_3, beacon_1_4), (beacon_2_3, beacon_2_4))) in processed_scanner.intersection(scanner_report) {
            if invariant_2 != invariant_1 {
                if let Some((beacon_1_2, beacon_1_1)) = try_match(beacon_1_1, beacon_1_2, beacon_1_3, beacon_1_4) {
                    let (beacon_2_2, beacon_2_1) = try_match(beacon_2_1, beacon_2_2, beacon_2_3, beacon_2_4).unwrap();

                    let orig_diff = beacon_1_2 - beacon_1_1;
                    let new_diff = beacon_2_2 - beacon_2_1;

                    let mut transformation_array = std::array::from_fn(|i| CoordinateTransformation::new(0, false));
                    for (i, new_coord) in new_diff.coordinates.iter().copied().enumerate() {
                        let old_coord_index = orig_diff.coordinates.iter().position(|it| it.abs() == new_coord.abs()).unwrap();
                        transformation_array[old_coord_index] = CoordinateTransformation::new(i, orig_diff.coordinates[old_coord_index] != new_coord)
                    }
                    let transformation = Vector3dTransformation::new(transformation_array);

                    let transformed_beacon_2_1 = transformation.transform(beacon_2_1);

                    let relative_scanner_position = beacon_1_1 - &transformed_beacon_2_1;

                    return Some((relative_scanner_position, transformation));
                }
            }
        }
    }

    return None;
}

fn try_match<'a>(beacon_1: &'a Vector3d, beacon_2: &'a Vector3d, beacon_3: &'a Vector3d, beacon_4: &'a Vector3d) -> Option<(&'a Vector3d, &'a Vector3d)> {
    if beacon_3 == beacon_1 || beacon_4 == beacon_1 {
        Some((beacon_2, beacon_1))
    } else if beacon_3 == beacon_2 || beacon_4 == beacon_2 {
        Some((beacon_1, beacon_2))
    } else {
        None
    }
}


#[derive(ParseYolo)]
#[pattern("{},{},{}")]
struct BeaconPosition {
    x: i32,
    y: i32,
    z: i32,
}

impl BeaconPosition {
    fn as_vector(&self) -> Vector3d {
        Vector3d::new(self.x, self.y, self.z)
    }
}

#[derive(new)]
struct ScannerReport {
    beacons: Vec<Vector3d>,
    graph: HashMap<DistanceInvariant, (usize, usize)>,
}

#[derive(Hash, Eq, PartialEq)]
struct DistanceInvariant {
    elements: [u32; 3],
}

impl DistanceInvariant {
    fn new(beacon_1: &Vector3d, beacon_2: &Vector3d) -> Self {
        let mut elements = beacon_1.abs_diff(beacon_2);
        elements.sort();
        Self { elements }
    }
}

#[derive(new)]
struct Vector3dTransformation {
    coordinates: [CoordinateTransformation; 3],
}

impl Vector3dTransformation {
    pub fn transform(&self, vector: &Vector3d) -> Vector3d {
        Vector3d::new(
            self.coordinates[0].transform(vector),
            self.coordinates[1].transform(vector),
            self.coordinates[2].transform(vector),
        )
    }
}

#[derive(new)]
struct CoordinateTransformation {
    coordinate: usize,
    invert: bool,
}

impl CoordinateTransformation {
    pub fn transform(&self, vector: &Vector3d) -> i32 {
        let transformed = vector.coordinates[self.coordinate];
        if self.invert {
            -transformed
        } else {
            transformed
        }
    }
}

#[derive(new)]
struct ProcessedScanner {
    scanner_position: Vector3d,
    report: ScannerReport,
}

impl ProcessedScanner {
    pub fn intersection<'a, 'b>(&'a self, other: &'b ScannerReport) -> impl Iterator<Item=((&'a DistanceInvariant, ((&'a Vector3d, &'a Vector3d), (&'b Vector3d, &'b Vector3d))))> {
        self.report.graph.iter()
            .filter_map(|(distance, &(my_beacon_index_1, my_beacon_index_2))|
                other.graph.get(distance).map(|&(other_beacon_index_1, other_beacon_index_2)| {
                    (
                        distance,
                        (
                            (&self.report.beacons[my_beacon_index_1], &self.report.beacons[my_beacon_index_2]),
                            (&other.beacons[other_beacon_index_1], &other.beacons[other_beacon_index_2])
                        )
                    )
                })
            )
    }

    pub fn contains(&self, beacon: &Vector3d) -> bool {
        (beacon - &self.scanner_position).iter().all(|it| it >= -1000 && it <= 1000)
    }
}


#[cfg(test)]
mod tests {
    use crate::input::InputData;

    use super::*;

    #[test]
    fn part_1_works() {
        let result = part_1(&data());

        assert_eq!(result, 79);
    }

    #[test]
    fn part_2_works() {
        let result = part_2(&data());

        assert_eq!(result, 3621);
    }

    fn data() -> InputData {
        InputData::from_string("
            --- scanner 0 ---
            404,-588,-901
            528,-643,409
            -838,591,734
            390,-675,-793
            -537,-823,-458
            -485,-357,347
            -345,-311,381
            -661,-816,-575
            -876,649,763
            -618,-824,-621
            553,345,-567
            474,580,667
            -447,-329,318
            -584,868,-557
            544,-627,-890
            564,392,-477
            455,729,728
            -892,524,684
            -689,845,-530
            423,-701,434
            7,-33,-71
            630,319,-379
            443,580,662
            -789,900,-551
            459,-707,401

            --- scanner 1 ---
            686,422,578
            605,423,415
            515,917,-361
            -336,658,858
            95,138,22
            -476,619,847
            -340,-569,-846
            567,-361,727
            -460,603,-452
            669,-402,600
            729,430,532
            -500,-761,534
            -322,571,750
            -466,-666,-811
            -429,-592,574
            -355,545,-477
            703,-491,-529
            -328,-685,520
            413,935,-424
            -391,539,-444
            586,-435,557
            -364,-763,-893
            807,-499,-711
            755,-354,-619
            553,889,-390

            --- scanner 2 ---
            649,640,665
            682,-795,504
            -784,533,-524
            -644,584,-595
            -588,-843,648
            -30,6,44
            -674,560,763
            500,723,-460
            609,671,-379
            -555,-800,653
            -675,-892,-343
            697,-426,-610
            578,704,681
            493,664,-388
            -671,-858,530
            -667,343,800
            571,-461,-707
            -138,-166,112
            -889,563,-600
            646,-828,498
            640,759,510
            -630,509,768
            -681,-892,-333
            673,-379,-804
            -742,-814,-386
            577,-820,562

            --- scanner 3 ---
            -589,542,597
            605,-692,669
            -500,565,-823
            -660,373,557
            -458,-679,-417
            -488,449,543
            -626,468,-788
            338,-750,-386
            528,-832,-391
            562,-778,733
            -938,-730,414
            543,643,-506
            -524,371,-870
            407,773,750
            -104,29,83
            378,-903,-323
            -778,-728,485
            426,699,580
            -438,-605,-362
            -469,-447,-387
            509,732,623
            647,635,-688
            -868,-804,481
            614,-800,639
            595,780,-596

            --- scanner 4 ---
            727,592,562
            -293,-554,779
            441,611,-461
            -714,465,-776
            -743,427,-804
            -660,-479,-426
            832,-632,460
            927,-485,-438
            408,393,-506
            466,436,-512
            110,16,151
            -258,-428,682
            -393,719,612
            -211,-452,876
            808,-476,-593
            -575,615,604
            -485,667,467
            -680,325,-822
            -627,-443,-432
            872,-547,-609
            833,512,582
            807,604,487
            839,-516,451
            891,-625,532
            -652,-548,-490
            30,-46,-14
        ")
    }
}