//! test cases for logistic regression

use anyhow::Result;
use ndarray::{array, Array1, Array2};

use glm_regress::{glm::Glm, logistic::Logistic, model::ModelBuilder};

#[test]
// this data caused an infinite loop with step halving
fn test_log_termination_0() -> Result<()> {
    let y: Array1<bool> = array![
        true, true, true, true, true, true, true, true, true, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false
    ];
    let x: Array2<f32> = Array2::from_shape_vec(
        (y.len(), 1),
        vec![
            0.26557046,
            0.5244869,
            -0.21809632,
            0.5479409,
            -0.3813186,
            0.03792116,
            -0.04919088,
            -0.453344,
            0.01047641,
            1.0668023,
            -0.18769538,
            -0.35202113,
            1.2880116,
            0.3166774,
            0.10186827,
            0.36787283,
            -0.24988532,
            -0.6783223,
            -0.7201773,
            -0.4314589,
            -0.5853845,
            -0.37470338,
            -0.75128376,
            1.2318513,
            -0.12531418,
            -0.42064115,
            -0.74604774,
            1.1256697,
            -0.08433908,
            1.3585893,
            0.005041659,
            0.18818086,
            -0.45354,
            -0.27919883,
            1.271002,
            -0.13111639,
            0.13094604,
            1.2829714,
            -0.29096013,
            1.2689505,
            0.3342812,
            0.08396453,
            1.1104087,
            -0.68441665,
            0.18637711,
            0.7208801,
            -0.50222147,
            1.0418514,
            -0.49237987,
            -0.5876201,
            -0.1234926,
            0.09748709,
            -0.2486701,
            -0.676539,
            0.20423794,
            -0.012358367,
            -0.642534,
            -0.59853345,
            0.14618611,
            -0.21990025,
            -0.7370253,
            1.0310516,
            0.7160105,
            1.0968622,
            -0.45908076,
            -0.5856943,
            1.2937942,
            -0.56300896,
            0.04820007,
            -0.0106089115,
            -0.14692777,
            -0.52543974,
            0.13696462,
            -0.59596336,
            0.24310935,
            -0.30030304,
            1.0555162,
            -0.3424111,
            0.5709101,
            1.3804396,
            1.2689683,
            0.18201947,
            -0.29528198,
            1.2367306,
            -0.22552526,
            -0.43786216,
            -0.14895535,
            -0.39894313,
            -0.7180385,
            0.28069985,
            0.19821835,
            1.252464,
            0.32849503,
            -0.7762929,
            -0.014078617,
            -0.45130554,
            -0.71616507,
            -0.6086625,
            -0.41315138,
            -0.5242159,
            -0.7438436,
            -0.81740195,
            0.55138636,
            1.3621999,
            -0.62829953,
            -0.70248127,
            -0.2101,
            -0.40677756,
            0.5391067,
            -0.66932964,
            0.7294785,
            -0.21406662,
            0.03267044,
            -0.6382973,
            -0.3233307,
            -0.13329303,
            0.3818912,
            1.2497455,
            0.4388262,
            -0.253937,
            -0.23217905,
            -0.5770677,
            0.41546327,
            0.53724766,
            -0.7986775,
            0.28877604,
            0.47571462,
            0.09451425,
            -0.34829602,
            -0.52060723,
            -0.49226674,
            -0.622851,
            1.2084823,
            0.6975992,
            -0.07615328,
            -0.5714911,
            -0.8744174,
            1.1694076,
            -0.65958846,
            -0.21230477,
            0.08752161,
            1.2759559,
            -0.45556974,
            0.5014289,
            -0.5371747,
            0.732478,
            0.33955103,
            -0.70246065,
            -0.37387347,
            -0.4731303,
            0.18484354,
            -0.3117661,
            1.2368045,
            0.7313858,
            0.64754677,
            -0.6285814,
            0.5638829,
            0.3041898,
            0.3807578,
            0.9275137,
            -0.7031316,
            0.1693201,
            0.25447702,
            0.4682963,
            -0.10173762,
            -0.07723445,
            0.40226156,
            -0.21446574,
            -0.4003486,
            0.3332528,
            -0.7320912,
            1.2398887,
            -0.4674436,
            0.25253803,
            1.0358511,
            0.5350432,
            -0.17243266,
            -0.6961477,
            0.30013,
            0.059309006,
            -0.4967607,
            -0.45456904,
            -0.56493205,
            1.1671963,
            0.31851083,
            -0.5102673,
            1.2568061,
            0.33830458,
            -0.07715261,
            0.3121574,
            -0.5151504,
            -0.16930008,
            -0.24594662,
            1.2807302,
            0.5825742,
            -0.7365061,
            -0.6639884,
            -0.6828925,
            0.5622875,
        ],
    )?;
    let off: Array1<f32> = array![
        -3.2022736, -3.5498385, -3.3002653, -2.906719, -3.390307, -1.0382099, -2.8323147,
        -3.415258, -2.1300457, -3.3786244, -3.601899, -3.1413345, -2.521451, -3.2458532,
        -3.3000097, -2.6364887, -2.2940626, -3.1264372, -3.5402062, -3.8959053, -3.2296875,
        -2.8795247, -3.6581087, -3.3581736, -3.716817, -3.4001236, -3.7475586, -2.8785355,
        -3.7822423, -2.5086546, -3.4469101, -2.6128473, -3.9081311, -3.1311884, -3.1878426,
        -3.2133822, -2.9195528, -3.1786385, -3.106109, -2.5597675, -4.0456214, -3.0524874,
        -3.8698661, -3.2513955, -3.5095043, -3.1672068, -4.1845937, -3.6488628, -3.413788,
        -3.249963, -3.9614666, -3.0888367, -3.6843758, -4.1181054, -2.4684455, -2.4847746,
        -4.1481447, -3.8704948, -2.9035354, -3.3992321, -3.5081766, -2.5176485, -3.4693534,
        -2.70371, -2.7795982, -2.1704588, -3.0139837, -2.0976164, -4.0200267, -3.2842436,
        -2.6681952, -3.1563013, -3.6118274, -2.7857578, -3.8758035, -4.309657, -3.731285,
        -4.1431813, -3.2412841, -2.2630973, -2.851879, -2.4832733, -2.9775357, -3.0602999,
        -3.2436771, -1.9410757, -3.1494207, -2.8839428, -3.3567767, -3.4017828, -2.5497537,
        -1.8616767, -2.7033262, -5.376764, -3.3350859, -3.2043877, -3.8789296, -2.9674404,
        -3.1814337, -2.8435996, -3.8597884, -4.263725, -2.1557596, -2.3115337, -3.3256352,
        -3.2682147, -3.2451437, -3.4969113, -3.088208, -3.1389472, -4.228744, -3.4586463,
        -2.6916747, -2.8399873, -2.5109015, -2.9670796, -2.5930202, -2.6308012, -3.0918543,
        -3.206477, -2.660061, -3.7584777, -3.1350126, -3.2037919, -4.063359, -3.6485202, -2.545376,
        -5.381941, -3.0109448, -2.844867, -3.532657, -4.1310296, -3.0159636, -3.1074188,
        -3.3314354, -2.5613039, -4.8554296, -2.3913429, -2.8213327, -3.1046772, -3.8812873,
        -3.0133724, -3.391033, -2.746602, -1.8625656, -3.1576986, -3.3705604, -3.4936805,
        -4.1623955, -5.2111564, -3.3146698, -2.6805778, -3.3225114, -3.783871, -3.2968557,
        -3.0687175, -3.36836, -3.7166808, -3.1068423, -3.2246234, -3.37831, -2.6144023, -3.8608716,
        -2.3788, -2.3303013, -3.2077525, -3.1591616, -2.874809, -2.617906, -2.8007066, -3.489993,
        -3.437864, -4.2981825, -3.5632074, -3.7131572, -2.7545307, -3.2097433, -3.3794284,
        -3.4762855, -1.9617656, -2.780623, -2.7300196, -2.710905, -3.904039, -2.85039, -3.4411893,
        -3.4012935, -4.5889945, -3.653743, -3.8879204, -3.566021, -3.8353205, -2.0306807,
        -2.944789, -3.4705598, -3.5821562, -2.8796465, -3.1926105, -2.709444
    ];
    let data = ModelBuilder::new(&y, &x).linear_offset(off).build()?;
    let fit = Logistic::regression(&data)?;
    dbg!(fit.result);
    dbg!(fit.n_iter);
    Ok(())
}

#[test]
// this data caused an infinite loop with step halving
fn test_log_termination_1() -> Result<()> {
    let y: Array1<bool> = array![
        true, true, true, true, true, true, true, true, true, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false
    ];
    let x: Array2<f32> = Array2::from_shape_vec(
        (y.len(), 1),
        vec![
            0.48353553,
            0.32687593,
            0.50492036,
            0.56624234,
            -0.14332628,
            -0.011929154,
            -0.4571042,
            0.65687394,
            0.015737057,
            0.23096406,
            0.012414932,
            -0.52334684,
            0.6178515,
            0.6964315,
            -1.0957825,
            -1.1722804,
            -1.384096,
            0.5960889,
            0.74327517,
            0.3723656,
            0.6944523,
            -1.1790563,
            -0.12711275,
            -1.2141193,
            -1.2183346,
            0.59808135,
            -0.051205873,
            0.06812978,
            -0.9532629,
            0.5170653,
            -1.2525051,
            -1.2220773,
            0.78843486,
            0.652954,
            -0.9228772,
            -0.46118212,
            -0.35449618,
            -1.2833532,
            -0.8604206,
            0.6273012,
            -1.017805,
            0.30743694,
            0.760401,
            0.5382639,
            -0.9086895,
            -0.7136542,
            0.5777776,
            -0.9980084,
            0.70232236,
            -0.44568497,
            -0.92240953,
            0.6939018,
            0.7389426,
            -1.1729985,
            -0.6568296,
            0.59121525,
            -0.43012452,
            -0.8200198,
            -0.7762546,
            -1.0411963,
            -0.45994437,
            0.62327325,
            -1.2486656,
            0.57346594,
            0.16317546,
            -0.7400616,
            -1.3026286,
            -0.016997218,
            -1.1019399,
            0.6611141,
            -1.1442604,
            0.70348763,
            -1.1237589,
            -0.40564853,
            0.78321004,
            -0.5143354,
            -0.7337447,
            0.8111557,
            -0.72404915,
            0.5662259,
            -1.3205436,
            0.61006343,
            0.6439452,
            -0.51819354,
            0.70379484,
            -1.4223874,
            0.66888523,
            -1.3163908,
            -0.58185303,
            -0.47290254,
            0.6307907,
            0.16947198,
            0.62836826,
            -0.9814276,
            -1.210118,
            0.52871215,
            0.7025646,
            0.67520285,
            -1.283294,
            -1.3194449,
            0.6918458,
            0.8273946,
            0.5684525,
            0.26348603,
            -0.012208343,
            0.6822369,
            -0.73920596,
            0.7346592,
            0.6920308,
            0.6592177,
            -0.6693195,
            0.72450817,
            0.6506053,
            0.5953907,
            -1.2699009,
            -0.30402076,
            -0.11133349,
            -1.3464684,
            0.6972213,
            -0.6777657,
            -0.71906555,
            0.45432138,
            -0.86802197,
            0.7041714,
            -0.3661058,
            0.7499423,
            -1.1538891,
            -0.99647,
            0.6569295,
            0.64912844,
            0.7465631,
            0.71838236,
            -1.264586,
            0.64894676,
            0.72622204,
            0.5509386,
            0.880937,
            -0.922426,
            0.63227654,
            0.6954663,
            0.7819948,
            -1.3025188,
            0.7222867,
            0.37153876,
            0.55291605,
            -1.2127208,
            0.7226932,
            0.7328142,
            0.74241686,
            0.93980014,
            0.72233117,
            0.64522636,
            -0.2788471,
            0.7747294,
            -1.1970979,
            -1.2959403,
            0.6387843,
            -0.8153173,
            -0.23521876,
            0.6858834,
            -0.679168,
            -1.3467581,
            0.7461469,
            -0.81385136,
            -1.0387502,
            -1.1294994,
            -0.9035834,
            0.6699456,
            -0.009785533,
            -0.053070188,
            -1.1826934,
            0.6465385,
            0.82264376,
            0.6977775,
            0.53678644,
            -0.6443374,
            -1.1633543,
            -1.0217475,
            0.73973286,
            0.56944954,
            0.30056667,
            0.08328712,
            -1.2830048,
            -1.1636153,
            -0.81747836,
            0.73330843,
            0.69177365,
            0.20494008,
            0.7062788,
            0.78206015,
            -1.0173594,
            -0.7045444,
            -1.4153104,
            0.6429255,
            -0.9160576,
            -0.5827174,
            -1.1231985,
            -0.58335847,
            -1.1834576,
        ],
    )?;
    let off: Array1<f32> = array![
        -3.2022738, -3.5498385, -3.3002653, -2.9067192, -3.3903067, -1.0382121, -2.8323152,
        -3.4152577, -2.1300468, -3.3786242, -3.6018987, -3.1413348, -2.5214515, -3.2458532,
        -3.3000097, -2.6364894, -2.2940636, -3.1264374, -3.540206, -3.8959048, -3.2296875,
        -2.8795252, -3.6581082, -3.3581736, -3.7168167, -3.4001236, -3.747558, -2.8785357,
        -3.782242, -2.5086553, -3.44691, -2.6128478, -3.9081306, -3.1311886, -3.1878426,
        -3.2133822, -2.919553, -3.1786385, -3.1061091, -2.5597682, -4.045621, -3.0524876,
        -3.8698654, -3.2513955, -3.509504, -3.1672068, -4.184593, -3.6488624, -3.4137878,
        -3.249963, -3.9614658, -3.088837, -3.6843753, -4.118105, -2.4684463, -2.4847755,
        -4.1481442, -3.8704944, -2.9035356, -3.399232, -3.5081763, -2.5176492, -3.4693534,
        -2.7037108, -2.7795987, -2.1704597, -3.0139842, -2.0976174, -4.020026, -3.2842436,
        -2.6681957, -3.1563013, -3.6118271, -2.7857583, -3.875803, -4.309656, -3.7312846,
        -4.143181, -3.2412841, -2.2630982, -2.8518794, -2.483274, -2.977536, -3.0603, -3.2436771,
        -1.9410769, -3.149421, -2.8839433, -3.3567767, -3.4017828, -2.5497541, -1.8616779,
        -2.7033267, -5.376762, -3.3350859, -3.204388, -3.8789291, -2.9674406, -3.181434, -2.8436,
        -3.859788, -4.263724, -2.1557608, -2.3115346, -3.3256352, -3.268215, -3.245144, -3.4969113,
        -3.0882082, -3.1389475, -4.2287436, -3.458646, -2.6916752, -2.8399875, -2.510902, -2.96708,
        -2.593021, -2.630802, -3.0918543, -3.206477, -2.6600616, -3.7584772, -3.1350129,
        -3.2037919, -4.0633583, -3.64852, -2.5453768, -5.381939, -3.010945, -2.8448672, -3.5326567,
        -4.1310287, -3.0159638, -3.107419, -3.3314354, -2.5613046, -4.855428, -2.3913436,
        -2.8213332, -3.1046772, -3.8812869, -3.0133727, -3.391033, -2.7466025, -1.8625668,
        -3.1576986, -3.3705604, -3.4936802, -4.1623945, -5.2111545, -3.3146696, -2.6805782,
        -3.3225114, -3.7838705, -3.2968557, -3.0687177, -3.36836, -3.7166803, -3.1068423,
        -3.2246234, -3.37831, -2.614403, -3.860871, -2.3788009, -2.330302, -3.2077525, -3.1591618,
        -2.8748093, -2.6179066, -2.800707, -3.4899929, -3.437864, -4.2981815, -3.5632071,
        -3.713157, -2.7545311, -3.2097433, -3.3794284, -3.4762852, -1.9617668, -2.7806234,
        -2.73002, -2.7109056, -3.9040384, -2.8503904, -3.4411893, -3.4012935, -4.5889935,
        -3.6537428, -3.88792, -3.5660207, -3.83532, -2.0306816, -2.9447892, -3.4705598, -3.5821557,
        -2.879647, -3.1926105, -2.7094445
    ];
    let data = ModelBuilder::new(&y, &x).linear_offset(off).build()?;
    let fit = Logistic::regression(&data)?;
    dbg!(fit.result);
    dbg!(fit.n_iter);
    Ok(())
}
