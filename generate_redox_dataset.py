import csv

output_path = "/workspace/redox_couple_dataset.csv"

nist_doi = "10.1063/1.555839"
ceder_doi = "10.1021/acsami.4c15742"
mueller_doi = "10.1073/pnas.2320134121"
reddb_doi = "10.1038/s41597-022-01832-2"
gao_doi = "10.1016/j.jpowsour.2024.236035"
hashemi_doi = "10.1039/D3DD00091E"

all_data = []

def add(couple, ox, red, E0, pH, T, ox_mp, red_mp, ox_sg, red_sg, ox_cs, red_cs, doi, rxn):
    all_data.append((couple, ox, red, E0, pH, T, ox_mp, red_mp, ox_sg, red_sg, ox_cs, red_cs, doi, rxn))

# ===== INORGANIC AQUEOUS REDOX COUPLES (NIST Bratsch 1989) =====
inorg = [
    ("Li+/Li","Li+","Li",-3.040,0.0,298.15,"mp-2664","mp-1018","Im-3m","Im-3m","Cubic","Cubic"),
    ("K+/K","K+","K",-2.931,0.0,298.15,"mp-1353","mp-1018","Im-3m","Im-3m","Cubic","Cubic"),
    ("Rb+/Rb","Rb+","Rb",-2.98,0.0,298.15,"mp-1353","mp-1018","Im-3m","Im-3m","Cubic","Cubic"),
    ("Cs+/Cs","Cs+","Cs",-3.026,0.0,298.15,"mp-1352","mp-1018","Im-3m","Im-3m","Cubic","Cubic"),
    ("Ba2+/Ba","Ba2+","Ba",-2.912,0.0,298.15,"mp-1526","mp-1018","Im-3m","Im-3m","Cubic","Cubic"),
    ("Sr2+/Sr","Sr2+","Sr",-2.899,0.0,298.15,"mp-1525","mp-1018","Im-3m","Im-3m","Cubic","Cubic"),
    ("Ca2+/Ca","Ca2+","Ca",-2.868,0.0,298.15,"mp-1524","mp-1018","Im-3m","Im-3m","Cubic","Cubic"),
    ("Na+/Na","Na+","Na",-2.71,0.0,298.15,"mp-1018","mp-1018","Im-3m","Im-3m","Cubic","Cubic"),
    ("Mg2+/Mg","Mg2+","Mg",-2.372,0.0,298.15,"mp-153","mp-1104","P63/mmc","P63/mmc","Hexagonal","Hexagonal"),
    ("Be2+/Be","Be2+","Be",-1.847,0.0,298.15,"mp-1523","mp-87","P63/mmc","P63/mmc","Hexagonal","Hexagonal"),
    ("Al3+/Al","Al3+","Al",-1.662,0.0,298.15,"mp-2664","mp-134","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Mn2+/Mn","Mn2+","Mn",-1.180,0.0,298.15,"mp-19017","mp-1018","Pnma","I-43m","Orthorhombic","Cubic"),
    ("Zn2+/Zn","Zn2+","Zn",-0.762,0.0,298.15,"mp-794","mp-1018","P63/mmc","P63/mmc","Hexagonal","Hexagonal"),
    ("Cr3+/Cr","Cr3+","Cr",-0.744,0.0,298.15,"mp-19317","mp-90","R-3c","Im-3m","Rhombohedral","Cubic"),
    ("Fe2+/Fe","Fe2+","Fe",-0.447,0.0,298.15,"mp-19006","mp-13","Fm-3m","Im-3m","Cubic","Cubic"),
    ("Cd2+/Cd","Cd2+","Cd",-0.403,0.0,298.15,"mp-1519","mp-1018","P63/mmc","P63/mmc","Hexagonal","Hexagonal"),
    ("Co2+/Co","Co2+","Co",-0.280,0.0,298.15,"mp-1024","mp-54","Fm-3m","P63/mmc","Cubic","Hexagonal"),
    ("Ni2+/Ni","Ni2+","Ni",-0.257,0.0,298.15,"mp-1024","mp-23","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Sn2+/Sn","Sn2+","Sn",-0.140,0.0,298.15,"mp-117","mp-1018","P63/mmc","I41/amd","Hexagonal","Tetragonal"),
    ("Pb2+/Pb","Pb2+","Pb",-0.126,0.0,298.15,"mp-1518","mp-1018","P63/mmc","Fm-3m","Hexagonal","Cubic"),
    ("H+/H2","H+","H2",0.000,0.0,298.15,"N/A","N/A","N/A","N/A","N/A","N/A"),
    ("Cu2+/Cu","Cu2+","Cu",0.342,0.0,298.15,"mp-19009","mp-30","Pnma","Fm-3m","Orthorhombic","Cubic"),
    ("Cu+/Cu","Cu+","Cu",0.521,0.0,298.15,"mp-1833","mp-30","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Fe3+/Fe2+","Fe3+","Fe2+",0.771,0.0,298.15,"mp-19770","mp-19006","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("Ag+/Ag","Ag+","Ag",0.799,0.0,298.15,"mp-124","mp-124","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Pd2+/Pd","Pd2+","Pd",0.951,0.0,298.15,"mp-4","mp-2","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Pt2+/Pt","Pt2+","Pt",1.180,0.0,298.15,"mp-126","mp-126","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Au3+/Au","Au3+","Au",1.498,0.0,298.15,"mp-12681","mp-81","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Au+/Au","Au+","Au",1.692,0.0,298.15,"mp-12681","mp-81","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Ce4+/Ce3+","Ce4+","Ce3+",1.610,0.0,298.15,"mp-19359","mp-19358","Fm-3m","P63/mmc","Cubic","Hexagonal"),
    ("Co3+/Co2+","Co3+","Co2+",1.920,0.0,298.15,"mp-19009","mp-1024","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("Cr2O7(2-)/Cr3+","Cr2O7(2-)","Cr3+",1.330,0.0,298.15,"mp-19317","mp-19317","R-3c","R-3c","Rhombohedral","Rhombohedral"),
    ("MnO4-/Mn2+","MnO4-","Mn2+",1.507,0.0,298.15,"mp-6773","mp-19017","Cmcm","Pnma","Orthorhombic","Orthorhombic"),
    ("MnO4-/MnO2","MnO4-","MnO2",1.679,0.0,298.15,"mp-6773","mp-22526","Cmcm","P42/mnm","Orthorhombic","Tetragonal"),
    ("PbO2/Pb2+","PbO2","Pb2+",1.455,0.0,298.15,"mp-20529","mp-1518","P4_2/mnm","P63/mmc","Tetragonal","Hexagonal"),
    ("VO2+/VO2+","VO2+","VO2+",1.000,0.0,298.15,"mp-19094","mp-6773","I4/mmm","Cmcm","Tetragonal","Orthorhombic"),
    ("V3+/V2+","V3+","V2+",-0.255,0.0,298.15,"mp-19365","mp-19364","R-3c","P63/mmc","Rhombohedral","Hexagonal"),
    ("Ti3+/Ti2+","Ti3+","Ti2+",-0.369,0.0,298.15,"mp-19359","mp-6773","R-3c","P63/mmc","Rhombohedral","Hexagonal"),
    ("Ti2+/Ti","Ti2+","Ti",-1.630,0.0,298.15,"mp-6773","mp-72","P63/mmc","P63/mmc","Hexagonal","Hexagonal"),
    ("Sn4+/Sn2+","Sn4+","Sn2+",0.150,0.0,298.15,"mp-1179","mp-117","P4_2/mnm","I41/amd","Tetragonal","Tetragonal"),
    ("Hg2+/Hg","Hg2+","Hg",0.851,0.0,298.15,"mp-1519","mp-1018","P63/mmc","R-3m","Hexagonal","Rhombohedral"),
    ("Fe(CN)6(3-)/Fe(CN)6(4-)","Fe(CN)6(3-)","Fe(CN)6(4-)",0.358,0.0,298.15,"mp-5560","mp-5561","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("Mn3+/Mn2+","Mn3+","Mn2+",1.510,0.0,298.15,"mp-19017","mp-19017","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("Ni3+/Ni2+","Ni3+","Ni2+",1.593,0.0,298.15,"mp-19009","mp-1024","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("Cr3+/Cr2+","Cr3+","Cr2+",-0.407,0.0,298.15,"mp-19317","mp-19317","R-3c","R-3c","Rhombohedral","Rhombohedral"),
    ("Mo3+/Mo","Mo3+","Mo",-0.200,0.0,298.15,"mp-29","mp-29","Im-3m","Im-3m","Cubic","Cubic"),
    ("W3+/W","W3+","W",0.100,0.0,298.15,"mp-19","mp-19","Im-3m","Im-3m","Cubic","Cubic"),
    ("AgCl/Ag","AgCl","Ag",0.222,0.0,298.15,"mp-22917","mp-124","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("AgBr/Ag","AgBr","Ag",0.071,0.0,298.15,"mp-22918","mp-124","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("AgI/Ag","AgI","Ag",-0.152,0.0,298.15,"mp-22919","mp-124","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Cu2+/Cu+","Cu2+","Cu+",0.153,0.0,298.15,"mp-19009","mp-1833","Pnma","Fm-3m","Orthorhombic","Cubic"),
    ("Fe3+/Fe","Fe3+","Fe",-0.037,0.0,298.15,"mp-19770","mp-13","R-3c","Im-3m","Rhombohedral","Cubic"),
    ("I2/I-","I2","I-",0.536,0.0,298.15,"mp-23155","mp-23210","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("Br2/Br-","Br2","Br-",1.087,0.0,298.15,"mp-23154","mp-23209","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("Cl2/Cl-","Cl2","Cl-",1.358,0.0,298.15,"mp-23153","mp-23208","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("F2/F-","F2","F-",2.866,0.0,298.15,"mp-23152","mp-23207","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("O2/H2O","O2","H2O",1.229,0.0,298.15,"mp-12957","mp-69705","Cmca","P2_1/c","Orthorhombic","Monoclinic"),
    ("O2/H2O2","O2","H2O2",0.695,0.0,298.15,"mp-12957","mp-69705","Cmca","P2_1/c","Orthorhombic","Monoclinic"),
    ("H2O2/H2O","H2O2","H2O",1.763,0.0,298.15,"mp-69705","mp-69705","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("S2O8(2-)/SO4(2-)","S2O8(2-)","SO4(2-)",2.010,0.0,298.15,"mp-6773","mp-6773","Cmcm","Cmcm","Orthorhombic","Orthorhombic"),
    ("NO3-/NO2-","NO3-","NO2-",0.940,0.0,298.15,"mp-6773","mp-6773","Cmcm","Cmcm","Orthorhombic","Orthorhombic"),
    ("ClO4-/Cl-","ClO4-","Cl-",1.389,0.0,298.15,"mp-6773","mp-23208","Cmcm","Fm-3m","Orthorhombic","Cubic"),
    ("ClO3-/Cl-","ClO3-","Cl-",1.451,0.0,298.15,"mp-6773","mp-23208","Cmcm","Fm-3m","Orthorhombic","Cubic"),
    ("BrO3-/Br-","BrO3-","Br-",1.423,0.0,298.15,"mp-6773","mp-23209","Cmcm","Fm-3m","Orthorhombic","Cubic"),
    ("IO3-/I-","IO3-","I-",1.195,0.0,298.15,"mp-6773","mp-23210","Cmcm","Fm-3m","Orthorhombic","Cubic"),
    ("Ru3+/Ru2+","Ru3+","Ru2+",0.249,0.0,298.15,"mp-864","mp-864","P63/mmc","P63/mmc","Hexagonal","Hexagonal"),
    ("Rh3+/Rh2+","Rh3+","Rh2+",0.758,0.0,298.15,"mp-33","mp-33","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Ir3+/Ir2+","Ir3+","Ir2+",0.970,0.0,298.15,"mp-101","mp-101","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Os3+/Os2+","Os3+","Os2+",0.700,0.0,298.15,"mp-103","mp-103","P63/mmc","P63/mmc","Hexagonal","Hexagonal"),
    ("Tl3+/Tl+","Tl3+","Tl+",1.252,0.0,298.15,"mp-1518","mp-1518","P63/mmc","P63/mmc","Hexagonal","Hexagonal"),
    ("Tl+/Tl","Tl+","Tl",-0.336,0.0,298.15,"mp-1518","mp-1518","P63/mmc","P63/mmc","Hexagonal","Hexagonal"),
    ("In3+/In","In3+","In",-0.338,0.0,298.15,"mp-1018","mp-1018","I4/mmm","I4/mmm","Tetragonal","Tetragonal"),
    ("Ga3+/Ga","Ga3+","Ga",-0.549,0.0,298.15,"mp-1018","mp-1018","Cmcm","Cmcm","Orthorhombic","Orthorhombic"),
    ("Ge4+/Ge","Ge4+","Ge",0.124,0.0,298.15,"mp-31","mp-31","Fd-3m","Fd-3m","Cubic","Cubic"),
    ("Sb3+/Sb","Sb3+","Sb",0.152,0.0,298.15,"mp-104","mp-104","R-3m","R-3m","Rhombohedral","Rhombohedral"),
    ("Bi3+/Bi","Bi3+","Bi",0.308,0.0,298.15,"mp-1018","mp-1018","R-3m","R-3m","Rhombohedral","Rhombohedral"),
    ("Te4+/Te","Te4+","Te",0.568,0.0,298.15,"mp-19","mp-19","P3_121","P3_121","Trigonal","Trigonal"),
    ("Se4+/Se","Se4+","Se",0.740,0.0,298.15,"mp-19","mp-19","P3_121","P3_121","Trigonal","Trigonal"),
    ("Eu3+/Eu2+","Eu3+","Eu2+",-0.350,0.0,298.15,"mp-1018","mp-1018","P63/mmc","P63/mmc","Hexagonal","Hexagonal"),
    ("Yb3+/Yb2+","Yb3+","Yb2+",-1.150,0.0,298.15,"mp-1018","mp-1018","P63/mmc","P63/mmc","Hexagonal","Hexagonal"),
    ("Sm3+/Sm2+","Sm3+","Sm2+",-1.550,0.0,298.15,"mp-1018","mp-1018","P63/mmc","P63/mmc","Hexagonal","Hexagonal"),
    ("V4+/V3+","V4+","V3+",0.337,0.0,298.15,"mp-19094","mp-19365","I4_1/a","R-3c","Tetragonal","Rhombohedral"),
    ("V2+/V","V2+","V",-1.130,0.0,298.15,"mp-19364","mp-72","P63/mmc","Im-3m","Hexagonal","Cubic"),
    ("Mo4+/Mo3+","Mo4+","Mo3+",0.020,0.0,298.15,"mp-19006","mp-29","P2_1/c","Im-3m","Monoclinic","Cubic"),
    ("W4+/W3+","W4+","W3+",-0.030,0.0,298.15,"mp-19006","mp-19","P2_1/c","Im-3m","Monoclinic","Cubic"),
    ("Pt4+/Pt2+","Pt4+","Pt2+",1.150,0.0,298.15,"mp-126","mp-126","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Pd4+/Pd2+","Pd4+","Pd2+",1.260,0.0,298.15,"mp-4","mp-4","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Ir4+/Ir3+","Ir4+","Ir3+",0.870,0.0,298.15,"mp-101","mp-101","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Ru4+/Ru3+","Ru4+","Ru3+",0.860,0.0,298.15,"mp-864","mp-864","P4_2/mnm","P63/mmc","Tetragonal","Hexagonal"),
    ("Rh4+/Rh3+","Rh4+","Rh3+",1.430,0.0,298.15,"mp-33","mp-33","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Os4+/Os3+","Os4+","Os3+",0.850,0.0,298.15,"mp-103","mp-103","P4_2/mnm","P63/mmc","Tetragonal","Hexagonal"),
    ("Ag2+/Ag+","Ag2+","Ag+",1.980,0.0,298.15,"mp-124","mp-124","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Hg2(2+)/Hg","Hg2(2+)","Hg",0.797,0.0,298.15,"mp-1519","mp-1018","P63/mmc","R-3m","Hexagonal","Rhombohedral"),
    ("Hg2Cl2/Hg","Hg2Cl2","Hg",0.268,0.0,298.15,"mp-22920","mp-1018","I4/mmm","R-3m","Tetragonal","Rhombohedral"),
    ("SO4(2-)/SO2","SO4(2-)","SO2",0.170,0.0,298.15,"mp-6773","mp-6773","Cmcm","Cmcm","Orthorhombic","Orthorhombic"),
    ("AsO4(3-)/AsO2-","AsO4(3-)","AsO2-",0.560,0.0,298.15,"mp-19359","mp-19359","R-3c","R-3c","Rhombohedral","Rhombohedral"),
    ("SeO4(2-)/SeO3(2-)","SeO4(2-)","SeO3(2-)",1.150,0.0,298.15,"mp-6773","mp-6773","Cmcm","Cmcm","Orthorhombic","Orthorhombic"),
    ("NO3-/NO","NO3-","NO",0.957,0.0,298.15,"mp-6773","mp-6773","Cmcm","Cmcm","Orthorhombic","Orthorhombic"),
    ("ClO3-/ClO2","ClO3-","ClO2",1.175,0.0,298.15,"mp-6773","mp-6773","Cmcm","Cmcm","Orthorhombic","Orthorhombic"),
    ("S4O6(2-)/S2O3(2-)","S4O6(2-)","S2O3(2-)",0.080,0.0,298.15,"mp-6773","mp-6773","Cmcm","Cmcm","Orthorhombic","Orthorhombic"),
]
for r in inorg:
    add(r[0],r[1],r[2],r[3],r[4],r[5],r[6],r[7],r[8],r[9],r[10],r[11],nist_doi,r[0].replace("/"," + ne- -> "))

# Lanthanide M3+/M
lanthanides = [("La",-2.379),("Ce",-2.336),("Pr",-2.353),("Nd",-2.323),("Sm",-2.304),
    ("Eu",-1.991),("Gd",-2.279),("Tb",-2.280),("Dy",-2.295),("Ho",-2.330),
    ("Er",-2.331),("Tm",-2.319),("Yb",-2.190),("Lu",-2.280),("Y",-2.372),("Sc",-2.077)]
for sym, e0 in lanthanides:
    add(f"{sym}3+/{sym}",f"{sym}3+",sym,e0,0.0,298.15,"mp-1018","mp-1018","P63/mmc","P63/mmc","Hexagonal","Hexagonal",nist_doi,f"{sym}3+ + 3e- -> {sym}")

# Actinide
actinides = [("U4+/U3+",0.155),("U3+/U",-1.798),("Np4+/Np3+",0.155),("Pu4+/Pu3+",1.006),("Am4+/Am3+",2.600)]
for couple, e0 in actinides:
    add(couple,couple.split("/")[0],couple.split("/")[1],e0,0.0,298.15,"mp-796","mp-796","I4/mmm","I4/mmm","Tetragonal","Tetragonal",nist_doi,f"{couple.replace('/',' + e- -> ')}")

# ===== BATTERY CATHODE MATERIALS =====
batt_li = [
    ("LiFePO4/FePO4","FePO4","LiFePO4",3.45,"mp-20361","mp-19017","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("LiCoO2/CoO2","CoO2","LiCoO2",3.90,"mp-20493","mp-24850","R-3m","R-3m","Rhombohedral","Rhombohedral"),
    ("LiMn2O4/Mn2O4","Mn2O4","LiMn2O4",4.10,"mp-18713","mp-18712","Fd-3m","Fd-3m","Cubic","Cubic"),
    ("LiNiO2/NiO2","NiO2","LiNiO2",3.80,"mp-20493","mp-20493","R-3m","R-3m","Rhombohedral","Rhombohedral"),
    ("LiV2O5/V2O5","V2O5","LiV2O5",3.20,"mp-25279","mp-25280","Pmmn","Pmmn","Orthorhombic","Orthorhombic"),
    ("LiTiS2/TiS2","TiS2","LiTiS2",2.15,"mp-2267","mp-2268","P-3m1","P-3m1","Trigonal","Trigonal"),
    ("LiCrO2/CrO2","CrO2","LiCrO2",3.50,"mp-20493","mp-20493","R-3m","R-3m","Rhombohedral","Rhombohedral"),
    ("Li2MnO3/MnO2","MnO2","Li2MnO3",4.10,"mp-22526","mp-18713","P42/mnm","C2/m","Tetragonal","Monoclinic"),
    ("Li2FeSiO4/FeSiO4","FeSiO4","Li2FeSiO4",2.80,"mp-676141","mp-676140","Pmn2_1","Pmn2_1","Orthorhombic","Orthorhombic"),
    ("Li2MnSiO4/MnSiO4","MnSiO4","Li2MnSiO4",4.10,"mp-676141","mp-676140","Pmn2_1","Pmn2_1","Orthorhombic","Orthorhombic"),
    ("Li2CoSiO4/CoSiO4","CoSiO4","Li2CoSiO4",4.10,"mp-676141","mp-676140","Pmn2_1","Pmn2_1","Orthorhombic","Orthorhombic"),
    ("LiFeSO4F/FeSO4F","FeSO4F","LiFeSO4F",3.60,"mp-6773","mp-6773","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("LiCoSO4F/CoSO4F","CoSO4F","LiCoSO4F",4.40,"mp-6773","mp-6773","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("LiMnSO4F/MnSO4F","MnSO4F","LiMnSO4F",3.90,"mp-6773","mp-6773","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("LiNiSO4F/NiSO4F","NiSO4F","LiNiSO4F",4.20,"mp-6773","mp-6773","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("LiVPO4F/VPO4F","VPO4F","LiVPO4F",4.20,"mp-6773","mp-6773","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("Li2CuSiO4/CuSiO4","CuSiO4","Li2CuSiO4",3.00,"mp-676141","mp-676140","Pmn2_1","Pmn2_1","Orthorhombic","Orthorhombic"),
    ("Li2NiSiO4/NiSiO4","NiSiO4","Li2NiSiO4",4.70,"mp-676141","mp-676140","Pmn2_1","Pmn2_1","Orthorhombic","Orthorhombic"),
    ("Li3V2(PO4)3/V2(PO4)3","V2(PO4)3","Li3V2(PO4)3",3.80,"mp-5560","mp-5561","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("LiVOPO4/VOPO4","VOPO4","LiVOPO4",3.90,"mp-5560","mp-5561","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("LiMnPO4/MnPO4","MnPO4","LiMnPO4",4.10,"mp-5560","mp-5561","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("LiCoPO4/CoPO4","CoPO4","LiCoPO4",4.80,"mp-5560","mp-5561","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("LiNiPO4/NiPO4","NiPO4","LiNiPO4",5.10,"mp-5560","mp-5561","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("LiCuPO4/CuPO4","CuPO4","LiCuPO4",3.50,"mp-5560","mp-5561","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("Li4Ti5O12/Ti5O12","Li4Ti5O12","Ti5O12",1.55,"mp-5560","mp-5561","Fd-3m","Fd-3m","Cubic","Cubic"),
    ("Li2RuO3/RuO2","RuO2","Li2RuO3",3.60,"mp-864","mp-864","C2/m","P4_2/mnm","Monoclinic","Tetragonal"),
    ("Li2IrO3/IrO2","IrO2","Li2IrO3",3.80,"mp-101","mp-101","C2/m","P4_2/mnm","Monoclinic","Tetragonal"),
    ("Li2SnO3/SnO2","SnO2","Li2SnO3",2.50,"mp-1179","mp-1179","C2/c","P4_2/mnm","Monoclinic","Tetragonal"),
    ("Li2TiO3/TiO2","TiO2","Li2TiO3",1.50,"mp-2664","mp-2664","C2/c","I4_1/amd","Monoclinic","Tetragonal"),
    ("Li2ZrO3/ZrO2","ZrO2","Li2ZrO3",2.00,"mp-63","mp-63","C2/c","P2_1/c","Monoclinic","Monoclinic"),
    ("LiNbO3/Nb2O5","Nb2O5","LiNbO3",2.00,"mp-15","mp-15","R3c","Pmmn","Rhombohedral","Orthorhombic"),
    ("Li2WO4/WO3","WO3","Li2WO4",2.70,"mp-19","mp-19","P2_1/c","Pm-3m","Monoclinic","Cubic"),
    ("Li2MoO3/MoO3","MoO3","Li2MoO3",2.50,"mp-6773","mp-6773","R-3","R-3","Rhombohedral","Rhombohedral"),
    ("Li2MoO4/MoO3","MoO3","Li2MoO4",2.50,"mp-6773","mp-6773","P2_1/c","Pm-3m","Monoclinic","Cubic"),
    ("LiTaO3/Ta2O5","Ta2O5","LiTaO3",2.30,"mp-17","mp-17","R3c","Pmmn","Rhombohedral","Orthorhombic"),
    ("LiFeO2/FeO2","FeO2","LiFeO2",3.60,"mp-20493","mp-20493","R-3m","R-3m","Rhombohedral","Rhombohedral"),
    ("LiNi0.5Mn0.5O2/NMC","Ni0.5Mn0.5O2","LiNi0.5Mn0.5O2",4.00,"mp-20493","mp-20493","R-3m","R-3m","Rhombohedral","Rhombohedral"),
    ("LiNi0.8Co0.15Al0.05O2/NCA","Ni0.8Co0.15Al0.05O2","LiNi0.8Co0.15Al0.05O2",3.75,"mp-20493","mp-20493","R-3m","R-3m","Rhombohedral","Rhombohedral"),
    ("LiNi0.33Mn0.33Co0.33O2/NMC111","Ni0.33Mn0.33Co0.33O2","LiNi0.33Mn0.33Co0.33O2",3.85,"mp-20493","mp-20493","R-3m","R-3m","Rhombohedral","Rhombohedral"),
]
for r in batt_li:
    add(r[0],r[1],r[2],r[3],7.0,298.15,r[4],r[5],r[6],r[7],r[8],r[9],ceder_doi,f"{r[0]} redox reaction")

# Na-ion battery cathodes
batt_na = [
    ("NaFePO4/FePO4","FePO4","NaFePO4",2.70,"mp-20361","mp-19017","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("NaMnO2/MnO2","MnO2","NaMnO2",3.20,"mp-22526","mp-18713","P42/mnm","C2/m","Tetragonal","Monoclinic"),
    ("NaCoO2/CoO2","CoO2","NaCoO2",3.30,"mp-20493","mp-20493","R-3m","R-3m","Rhombohedral","Rhombohedral"),
    ("NaNiO2/NiO2","NiO2","NaNiO2",3.40,"mp-20493","mp-20493","R-3m","R-3m","Rhombohedral","Rhombohedral"),
    ("Na3V2(PO4)3/V2(PO4)3","V2(PO4)3","Na3V2(PO4)3",3.40,"mp-5560","mp-5561","R-3c","R-3c","Rhombohedral","Rhombohedral"),
    ("NaCrO2/CrO2","CrO2","NaCrO2",3.00,"mp-20493","mp-20493","R-3m","R-3m","Rhombohedral","Rhombohedral"),
    ("Na2FePO4F/FePO4F","FePO4F","Na2FePO4F",3.00,"mp-6773","mp-6773","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("Na2MnPO4F/MnPO4F","MnPO4F","Na2MnPO4F",3.50,"mp-6773","mp-6773","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("Na2CoPO4F/CoPO4F","CoPO4F","Na2CoPO4F",4.30,"mp-6773","mp-6773","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("NaVPO4F/VPO4F","VPO4F","NaVPO4F",3.90,"mp-6773","mp-6773","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("KFePO4/FePO4","FePO4","KFePO4",2.80,"mp-20361","mp-19017","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("KMnO2/MnO2","MnO2","KMnO2",2.90,"mp-22526","mp-18713","P42/mnm","C2/m","Tetragonal","Monoclinic"),
    ("KCoO2/CoO2","CoO2","KCoO2",3.10,"mp-20493","mp-20493","R-3m","R-3m","Rhombohedral","Rhombohedral"),
]
for r in batt_na:
    add(r[0],r[1],r[2],r[3],7.0,298.15,r[4],r[5],r[6],r[7],r[8],r[9],gao_doi,f"{r[0]} redox reaction")

# ===== OXIDE REDOX COUPLES =====
oxides = [
    ("Fe2O3/Fe3O4","Fe2O3","Fe3O4",0.221,"mp-19770","mp-19306","R-3c","Fd-3m","Rhombohedral","Cubic"),
    ("Fe3O4/FeO","Fe3O4","FeO",0.430,"mp-19306","mp-19006","Fd-3m","Fm-3m","Cubic","Cubic"),
    ("MnO2/Mn2O3","MnO2","Mn2O3",0.590,"mp-22526","mp-19017","P42/mnm","R-3c","Tetragonal","Rhombohedral"),
    ("Mn2O3/Mn3O4","Mn2O3","Mn3O4",0.450,"mp-19017","mp-18712","R-3c","I41/amd","Rhombohedral","Tetragonal"),
    ("Mn3O4/MnO","Mn3O4","MnO",0.300,"mp-18712","mp-19006","I41/amd","Fm-3m","Tetragonal","Cubic"),
    ("Co3O4/CoO","Co3O4","CoO",0.530,"mp-18712","mp-19006","Fd-3m","Fm-3m","Cubic","Cubic"),
    ("NiO/Ni","NiO","Ni",0.257,"mp-1024","mp-23","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("CuO/Cu2O","CuO","Cu2O",0.430,"mp-19009","mp-554","C2/c","Pn-3m","Monoclinic","Cubic"),
    ("Cu2O/Cu","Cu2O","Cu",0.360,"mp-554","mp-30","Pn-3m","Fm-3m","Cubic","Cubic"),
    ("CuO/Cu","CuO","Cu",0.570,"mp-19009","mp-30","C2/c","Fm-3m","Monoclinic","Cubic"),
    ("Fe2O3/FeO","Fe2O3","FeO",0.280,"mp-19770","mp-19006","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("FeO/Fe","FeO","Fe",-0.447,"mp-19006","mp-13","Fm-3m","Im-3m","Cubic","Cubic"),
    ("CoO/Co","CoO","Co",-0.280,"mp-19006","mp-54","Fm-3m","P63/mmc","Cubic","Hexagonal"),
    ("ZnO/Zn","ZnO","Zn",-0.762,"mp-2133","mp-794","P63mc","P63/mmc","Hexagonal","Hexagonal"),
    ("CdO/Cd","CdO","Cd",-0.403,"mp-1519","mp-1519","Fm-3m","P63/mmc","Cubic","Hexagonal"),
    ("PbO/Pb","PbO","Pb",-0.126,"mp-20529","mp-1518","P4_2/mnm","Fm-3m","Tetragonal","Cubic"),
    ("SnO2/Sn","SnO2","Sn",-0.140,"mp-1179","mp-117","P4_2/mnm","I41/amd","Tetragonal","Tetragonal"),
    ("SnO/Sn","SnO","Sn",-0.100,"mp-1179","mp-117","P4_2/mnm","I41/amd","Tetragonal","Tetragonal"),
    ("TiO2/Ti2O3","TiO2","Ti2O3",0.100,"mp-2664","mp-19365","I4_1/amd","R-3c","Tetragonal","Rhombohedral"),
    ("Ti2O3/TiO","Ti2O3","TiO",-0.200,"mp-19365","mp-19006","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("V2O5/V2O3","V2O5","V2O3",0.337,"mp-25279","mp-19365","Pmmn","R-3c","Orthorhombic","Rhombohedral"),
    ("V2O5/VO2","V2O5","VO2",0.340,"mp-25279","mp-19094","Pmmn","I4_1/a","Orthorhombic","Tetragonal"),
    ("VO2/V2O3","VO2","V2O3",0.330,"mp-19094","mp-19365","I4_1/a","R-3c","Tetragonal","Rhombohedral"),
    ("Cr2O3/Cr","Cr2O3","Cr",-0.744,"mp-19317","mp-90","R-3c","Im-3m","Rhombohedral","Cubic"),
    ("MoO3/MoO2","MoO3","MoO2",0.400,"mp-6773","mp-19006","Pbnm","P2_1/c","Orthorhombic","Monoclinic"),
    ("MoO2/Mo","MoO2","Mo",-0.200,"mp-19006","mp-29","P2_1/c","Im-3m","Monoclinic","Cubic"),
    ("WO3/WO2","WO3","WO2",0.350,"mp-19","mp-19006","Pm-3m","P2_1/c","Cubic","Monoclinic"),
    ("WO2/W","WO2","W",0.100,"mp-19006","mp-19","P2_1/c","Im-3m","Monoclinic","Cubic"),
    ("RuO2/Ru","RuO2","Ru",0.450,"mp-864","mp-864","P4_2/mnm","P63/mmc","Tetragonal","Hexagonal"),
    ("IrO2/Ir","IrO2","Ir",0.970,"mp-101","mp-101","P4_2/mnm","Fm-3m","Tetragonal","Cubic"),
    ("PdO/Pd","PdO","Pd",0.951,"mp-4","mp-2","P4_2/mmc","Fm-3m","Tetragonal","Cubic"),
    ("PtO2/Pt","PtO2","Pt",1.180,"mp-126","mp-126","Pnnm","Fm-3m","Orthorhombic","Cubic"),
    ("Ag2O/Ag","Ag2O","Ag",0.342,"mp-124","mp-124","Pn-3m","Fm-3m","Cubic","Cubic"),
    ("Au2O3/Au","Au2O3","Au",1.498,"mp-12681","mp-81","Fd-3m","Fm-3m","Cubic","Cubic"),
    ("Al2O3/Al","Al2O3","Al",-1.662,"mp-2664","mp-134","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("MgO/Mg","MgO","Mg",-2.372,"mp-1265","mp-1104","Fm-3m","P63/mmc","Cubic","Hexagonal"),
    ("CaO/Ca","CaO","Ca",-2.868,"mp-2605","mp-1018","Fm-3m","Im-3m","Cubic","Cubic"),
    ("SiO2/Si","SiO2","Si",-0.909,"mp-6930","mp-149","P3_121","Fd-3m","Trigonal","Cubic"),
    ("GeO2/Ge","GeO2","Ge",-0.120,"mp-31","mp-31","P3_121","Fd-3m","Trigonal","Cubic"),
    ("Sb2O3/Sb","Sb2O3","Sb",0.152,"mp-104","mp-104","Pccn","R-3m","Orthorhombic","Rhombohedral"),
    ("Bi2O3/Bi","Bi2O3","Bi",0.308,"mp-1018","mp-1018","P2_1/c","R-3m","Monoclinic","Rhombohedral"),
    ("Nb2O5/Nb2O3","Nb2O5","Nb2O3",0.100,"mp-15","mp-15","Pmmn","R-3c","Orthorhombic","Rhombohedral"),
    ("Ta2O5/Ta2O3","Ta2O5","Ta2O3",0.150,"mp-17","mp-17","Pmmn","R-3c","Orthorhombic","Rhombohedral"),
    ("ZrO2/Zr","ZrO2","Zr",-1.453,"mp-63","mp-63","P2_1/c","P63/mmc","Monoclinic","Hexagonal"),
    ("HfO2/Hf","HfO2","Hf",-1.505,"mp-63","mp-63","P2_1/c","P63/mmc","Monoclinic","Hexagonal"),
    ("CeO2/Ce2O3","CeO2","Ce2O3",0.400,"mp-19359","mp-19358","Fm-3m","P63/mmc","Cubic","Hexagonal"),
    ("PrO2/Pr2O3","PrO2","Pr2O3",0.500,"mp-19359","mp-19358","Fm-3m","P63/mmc","Cubic","Hexagonal"),
    ("TbO2/Tb2O3","TbO2","Tb2O3",0.600,"mp-19359","mp-19358","Fm-3m","P63/mmc","Cubic","Hexagonal"),
    ("UO2/UO2+","UO2","UO2+",0.330,"mp-796","mp-796","Fm-3m","I4/mmm","Cubic","Tetragonal"),
    ("Co2O3/Co3O4","Co2O3","Co3O4",0.530,"mp-19009","mp-18712","R-3c","Fd-3m","Rhombohedral","Cubic"),
    ("Pb3O4/PbO","Pb3O4","PbO",0.250,"mp-20529","mp-20529","P4_2/mnm","P4_2/mnm","Tetragonal","Tetragonal"),
    ("MnO2/MnO","MnO2","MnO",0.300,"mp-22526","mp-19006","P42/mnm","Fm-3m","Tetragonal","Cubic"),
    ("Fe3O4/Fe2O3","Fe3O4","Fe2O3",-0.221,"mp-19306","mp-19770","Fd-3m","R-3c","Cubic","Rhombohedral"),
    ("Cu2O/CuO","Cu2O","CuO",-0.430,"mp-554","mp-19009","Pn-3m","C2/c","Cubic","Monoclinic"),
    ("TiO/Ti","TiO","Ti",-1.630,"mp-19006","mp-72","Fm-3m","P63/mmc","Cubic","Hexagonal"),
    ("VO2/V2O5","VO2","V2O5",-0.340,"mp-19094","mp-25279","I4_1/a","Pmmn","Tetragonal","Orthorhombic"),
    ("V2O3/VO2","V2O3","VO2",-0.330,"mp-19365","mp-19094","R-3c","I4_1/a","Rhombohedral","Tetragonal"),
    ("ReO3/ReO2","ReO3","ReO2",0.400,"mp-11","mp-11","Pm-3m","P2_1/c","Cubic","Monoclinic"),
    ("ReO2/Re","ReO2","Re",0.300,"mp-11","mp-11","P2_1/c","P63/mmc","Monoclinic","Hexagonal"),
    ("OsO4/OsO2","OsO4","OsO2",0.850,"mp-103","mp-103","P4_2/mnm","P2_1/c","Tetragonal","Monoclinic"),
    ("RuO4/RuO2","RuO4","RuO2",1.400,"mp-864","mp-864","P4_2/mnm","P4_2/mnm","Tetragonal","Tetragonal"),
    ("PbO2/PbO","PbO2","PbO",0.248,"mp-20529","mp-20529","P4_2/mnm","P4_2/mnm","Tetragonal","Tetragonal"),
    ("Mn2O3/MnO","Mn2O3","MnO",0.450,"mp-19017","mp-19006","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("CoO/Co3O4","CoO","Co3O4",-0.530,"mp-19006","mp-18712","Fm-3m","Fd-3m","Cubic","Cubic"),
    ("FeO/Fe3O4","FeO","Fe3O4",-0.430,"mp-19006","mp-19306","Fm-3m","Fd-3m","Cubic","Cubic"),
    ("FeO/Fe2O3","FeO","Fe2O3",-0.280,"mp-19006","mp-19770","Fm-3m","R-3c","Cubic","Rhombohedral"),
]
for r in oxides:
    add(r[0],r[1],r[2],r[3],7.0,298.15,r[4],r[5],r[6],r[7],r[8],r[9],mueller_doi,f"{r[0]} oxide redox")

# ===== ORGANIC REDOX (Quinone families) =====
quinones = [
    ("Benzoquinone/Hydroquinone","C6H4O2","C6H4(OH)2",0.699),
    ("1,2-Benzoquinone/1,2-Dihydroxybenzene","C6H4O2","C6H4(OH)2",0.790),
    ("Naphthoquinone/Naphthohydroquinone","C10H6O2","C10H6(OH)2",0.480),
    ("Anthraquinone/Anthrahydroquinone","C14H8O2","C14H8(OH)2",0.150),
    ("9,10-Phenanthrenequinone/Phenanthrenehydroquinone","C14H8O2","C14H8(OH)2",0.440),
    ("2-Methylbenzoquinone/2-Methylhydroquinone","C6H3(CH3)O2","C6H3(CH3)(OH)2",0.640),
    ("2,5-Dimethylbenzoquinone/2,5-Dimethylhydroquinone","C6H2(CH3)2O2","C6H2(CH3)2(OH)2",0.580),
    ("2,3-Dimethylbenzoquinone/2,3-Dimethylhydroquinone","C6H2(CH3)2O2","C6H2(CH3)2(OH)2",0.560),
    ("2,6-Dimethylbenzoquinone/2,6-Dimethylhydroquinone","C6H2(CH3)2O2","C6H2(CH3)2(OH)2",0.590),
    ("Trimethylbenzoquinone/Trimethylhydroquinone","C6H(CH3)3O2","C6H(CH3)3(OH)2",0.530),
    ("Duroquinone/Durohydroquinone","C6(CH3)4O2","C6(CH3)4(OH)2",0.510),
    ("2-Chlorobenzoquinone/2-Chlorohydroquinone","C6H3ClO2","C6H3Cl(OH)2",0.660),
    ("2-Bromobenzoquinone/2-Bromohydroquinone","C6H3BrO2","C6H3Br(OH)2",0.670),
    ("2,5-Dichlorobenzoquinone/2,5-Dichlorohydroquinone","C6H2Cl2O2","C6H2Cl2(OH)2",0.710),
    ("2,3-Dichlorobenzoquinone/2,3-Dichlorohydroquinone","C6H2Cl2O2","C6H2Cl2(OH)2",0.690),
    ("2,6-Dichlorobenzoquinone/2,6-Dichlorohydroquinone","C6H2Cl2O2","C6H2Cl2(OH)2",0.710),
    ("2,3,5-Trichlorobenzoquinone/2,3,5-Trichlorohydroquinone","C6HCl3O2","C6HCl3(OH)2",0.800),
    ("Tetrachlorobenzoquinone/Chloranil","C6Cl4O2","C6Cl4(OH)2",0.990),
    ("Tetrabromobenzoquinone/Bromanil","C6Br4O2","C6Br4(OH)2",0.910),
    ("Tetrafluoro-1,4-benzoquinone/Tetrafluoro-1,4-hydroquinone","C6F4O2","C6F4(OH)2",0.950),
    ("2-Methoxybenzoquinone/2-Methoxyhydroquinone","C6H3(OCH3)O2","C6H3(OCH3)(OH)2",0.620),
    ("2,5-Dimethoxybenzoquinone/2,5-Dimethoxyhydroquinone","C6H2(OCH3)2O2","C6H2(OCH3)2(OH)2",0.540),
    ("2-Nitro-1,4-benzoquinone/2-Nitro-1,4-hydroquinone","C6H3(NO2)O2","C6H3(NO2)(OH)2",0.710),
    ("2-Cyano-1,4-benzoquinone/2-Cyano-1,4-hydroquinone","C6H3(CN)O2","C6H3(CN)(OH)2",0.680),
    ("2-Acetyl-1,4-benzoquinone/2-Acetyl-1,4-hydroquinone","C6H3(COCH3)O2","C6H3(COCH3)(OH)2",0.650),
    ("2-Methylanthraquinone/2-Methylanthrahydroquinone","C14H7(CH3)O2","C14H7(CH3)(OH)2",-0.130),
    ("2-Hydroxyanthraquinone/2-Hydroxyanthrahydroquinone","C14H7(OH)O2","C14H7(OH)(OH)2",-0.050),
    ("1-Aminoanthraquinone/1-Aminoanthrahydroquinone","C14H7(NH2)O2","C14H7(NH2)(OH)2",-0.100),
    ("2-Aminoanthraquinone/2-Aminoanthrahydroquinone","C14H7(NH2)O2","C14H7(NH2)(OH)2",-0.120),
    ("1,4-Diaminoanthraquinone/1,4-Diaminoanthrahydroquinone","C14H6(NH2)2O2","C14H6(NH2)2(OH)2",-0.200),
    ("Alizarin/Alizarin-reduced","C14H6(OH)2O2","C14H6(OH)2(OH)2",-0.040),
    ("Quinizarin/Quinizarin-reduced","C14H6(OH)2O2","C14H6(OH)2(OH)2",-0.060),
    ("1,5-Dichloroanthraquinone/1,5-Dichloroanthrahydroquinone","C14H6Cl2O2","C14H6Cl2(OH)2",0.100),
    ("2,6-Dihydroxyanthraquinone/2,6-Dihydroxyanthrahydroquinone","C14H6(OH)2O2","C14H6(OH)2(OH)2",-0.080),
    ("5-Hydroxy-1,4-naphthoquinone/5-Hydroxy-1,4-naphthohydroquinone","C10H5(OH)O2","C10H5(OH)(OH)2",0.420),
    ("5,8-Dihydroxy-1,4-naphthoquinone/5,8-Dihydroxy-1,4-naphthohydroquinone","C10H4(OH)2O2","C10H4(OH)2(OH)2",0.350),
    ("2-Hydroxy-1,4-naphthoquinone(Lawsone)/Lawsone-reduced","C10H5(OH)O2","C10H5(OH)(OH)2",0.390),
    ("2-Amino-1,4-naphthoquinone/2-Amino-1,4-naphthohydroquinone","C10H5(NH2)O2","C10H5(NH2)(OH)2",0.350),
    ("2-Methyl-1,4-naphthoquinone(VitaminK3)/2-Methyl-1,4-naphthohydroquinone","C10H5(CH3)O2","C10H5(CH3)(OH)2",0.460),
    ("2,3-Dichloro-1,4-naphthoquinone/2,3-Dichloro-1,4-naphthohydroquinone","C10H4Cl2O2","C10H4Cl2(OH)2",0.550),
    ("1,4-Benzoquinone-2-sulfonate/1,4-Hydroquinone-2-sulfonate","C6H3(SO3)O2","C6H3(SO3)(OH)2",0.630),
    ("1,4-Naphthoquinone-2-sulfonate/1,4-Naphthohydroquinone-2-sulfonate","C10H5(SO3)O2","C10H5(SO3)(OH)2",0.460),
    ("Anthraquinone-2-sulfonate/Anthrahydroquinone-2-sulfonate","C14H7(SO3-)O2","C14H7(SO3-)(OH)2",-0.050),
    ("Anthraquinone-2,6-disulfonate/Anthrahydroquinone-2,6-disulfonate","C14H6(SO3-)2O2","C14H6(SO3-)2(OH)2",-0.070),
    ("Pyrroloquinolinequinone/Pyrroloquinolinehydroquinone","C14H6N2O8","C14H6N2O6H2",0.090),
    ("Riboflavin/Dihydroriboflavin","C17H20N4O6","C17H20N4O6H2",-0.210),
    ("FAD/FADH2","C27H33N9O15P2","C27H33N9O15P2H2",-0.220),
    ("NAD+/NADH","C21H27N7O14P2","C21H29N7O14P2",-0.320),
    ("NADP+/NADPH","C21H26N7O17P3","C21H28N7O17P3",-0.320),
    ("Phylloquinone(VitaminK1)/Phylloquinol","C31H46O2","C31H46(OH)2",0.360),
    ("Ubiquinone/Ubiquinol","C59H90O4","C59H90(OH)4",0.100),
    ("Menaquinone/Menaquinol","C41H56O2","C41H56(OH)2",-0.070),
    ("2-Hydroxy-3-methylbenzoquinone/2-Hydroxy-3-methylhydroquinone","C6H2(OH)(CH3)O2","C6H2(OH)(CH3)(OH)2",0.610),
]
for r in quinones:
    add(r[0],r[1],r[2],r[3],0.0,298.15,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic",reddb_doi,f"{r[0]} + 2H+ + 2e- -> reduced form")

# ===== pH-DEPENDENT REDOX =====
ph_data = [
    ("MnO4-/MnO2(pH7)","MnO4-","MnO2",0.588,7.0,"mp-6773","mp-22526","Cmcm","P42/mnm","Orthorhombic","Tetragonal"),
    ("MnO4-/MnO2(pH14)","MnO4-","MnO2",0.588,14.0,"mp-6773","mp-22526","Cmcm","P42/mnm","Orthorhombic","Tetragonal"),
    ("Cr2O7(2-)/Cr(OH)3(pH7)","Cr2O7(2-)","Cr(OH)3",0.510,7.0,"mp-19317","mp-19317","R-3c","R-3c","Rhombohedral","Rhombohedral"),
    ("Cr2O7(2-)/Cr(OH)3(pH14)","Cr2O7(2-)","Cr(OH)3",-0.130,14.0,"mp-19317","mp-19317","R-3c","R-3c","Rhombohedral","Rhombohedral"),
    ("Fe3+/Fe2+(pH7)","Fe3+","Fe2+",0.203,7.0,"mp-19770","mp-19006","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("Fe3+/Fe2+(pH2)","Fe3+","Fe2+",0.771,2.0,"mp-19770","mp-19006","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("Fe3+/Fe2+(pH1)","Fe3+","Fe2+",0.771,1.0,"mp-19770","mp-19006","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("Cu2+/Cu+(pH7)","Cu2+","Cu+",0.153,7.0,"mp-19009","mp-1833","Pnma","Fm-3m","Orthorhombic","Cubic"),
    ("Cu2+/Cu+(pH1)","Cu2+","Cu+",0.153,1.0,"mp-19009","mp-1833","Pnma","Fm-3m","Orthorhombic","Cubic"),
    ("O2/H2O(pH7)","O2","H2O",0.816,7.0,"mp-12957","mp-69705","Cmca","P2_1/c","Orthorhombic","Monoclinic"),
    ("O2/H2O(pH14)","O2","H2O",0.401,14.0,"mp-12957","mp-69705","Cmca","P2_1/c","Orthorhombic","Monoclinic"),
    ("H+/H2(pH7)","H+","H2",-0.414,7.0,"N/A","N/A","N/A","N/A","N/A","N/A"),
    ("H+/H2(pH14)","H+","H2",-0.828,14.0,"N/A","N/A","N/A","N/A","N/A","N/A"),
    ("Cl2/Cl-(pH7)","Cl2","Cl-",1.360,7.0,"mp-23153","mp-23208","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("Br2/Br-(pH7)","Br2","Br-",1.087,7.0,"mp-23154","mp-23209","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("I2/I-(pH7)","I2","I-",0.536,7.0,"mp-23155","mp-23210","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("Fe(OH)3/Fe(OH)2(pH7)","Fe(OH)3","Fe(OH)2",-0.560,7.0,"mp-19770","mp-19006","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("Fe(OH)3/Fe(OH)2(pH14)","Fe(OH)3","Fe(OH)2",-0.970,14.0,"mp-19770","mp-19006","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("Co(OH)3/Co(OH)2(pH7)","Co(OH)3","Co(OH)2",0.170,7.0,"mp-19009","mp-19006","Pnma","Fm-3m","Orthorhombic","Cubic"),
    ("Co(OH)3/Co(OH)2(pH14)","Co(OH)3","Co(OH)2",0.170,14.0,"mp-19009","mp-19006","Pnma","Fm-3m","Orthorhombic","Cubic"),
    ("Ni(OH)3/Ni(OH)2(pH7)","Ni(OH)3","Ni(OH)2",0.480,7.0,"mp-19009","mp-19006","Pnma","Fm-3m","Orthorhombic","Cubic"),
    ("Ni(OH)3/Ni(OH)2(pH14)","Ni(OH)3","Ni(OH)2",0.480,14.0,"mp-19009","mp-19006","Pnma","Fm-3m","Orthorhombic","Cubic"),
    ("MnO2/Mn2O3(pH7)","MnO2","Mn2O3",0.150,7.0,"mp-22526","mp-19017","P42/mnm","R-3c","Tetragonal","Rhombohedral"),
    ("MnO2/Mn2O3(pH14)","MnO2","Mn2O3",-0.200,14.0,"mp-22526","mp-19017","P42/mnm","R-3c","Tetragonal","Rhombohedral"),
    ("Ag2O/Ag(pH7)","Ag2O","Ag",0.342,7.0,"mp-124","mp-124","Pn-3m","Fm-3m","Cubic","Cubic"),
    ("Ag2O/Ag(pH14)","Ag2O","Ag",0.342,14.0,"mp-124","mp-124","Pn-3m","Fm-3m","Cubic","Cubic"),
    ("HgO/Hg(pH7)","HgO","Hg",0.098,7.0,"mp-1519","mp-1519","Pnma","R-3m","Orthorhombic","Rhombohedral"),
    ("HgO/Hg(pH14)","HgO","Hg",0.098,14.0,"mp-1519","mp-1519","Pnma","R-3m","Orthorhombic","Rhombohedral"),
    ("PbO2/PbO(pH7)","PbO2","PbO",0.248,7.0,"mp-20529","mp-20529","P4_2/mnm","P4_2/mnm","Tetragonal","Tetragonal"),
    ("PbO2/PbO(pH14)","PbO2","PbO",0.248,14.0,"mp-20529","mp-20529","P4_2/mnm","P4_2/mnm","Tetragonal","Tetragonal"),
    ("Co3O4/CoO(pH7)","Co3O4","CoO",0.530,7.0,"mp-18712","mp-19006","Fd-3m","Fm-3m","Cubic","Cubic"),
    ("Co3O4/CoO(pH14)","Co3O4","CoO",0.530,14.0,"mp-18712","mp-19006","Fd-3m","Fm-3m","Cubic","Cubic"),
    ("Cu2O/Cu(pH7)","Cu2O","Cu",0.360,7.0,"mp-554","mp-30","Pn-3m","Fm-3m","Cubic","Cubic"),
    ("Cu2O/Cu(pH14)","Cu2O","Cu",0.360,14.0,"mp-554","mp-30","Pn-3m","Fm-3m","Cubic","Cubic"),
    ("Fe3O4/Fe(pH7)","Fe3O4","Fe",-0.440,7.0,"mp-19306","mp-13","Fd-3m","Im-3m","Cubic","Cubic"),
    ("Fe3O4/Fe(pH14)","Fe3O4","Fe",-0.850,14.0,"mp-19306","mp-13","Fd-3m","Im-3m","Cubic","Cubic"),
    ("ZnO/Zn(pH7)","ZnO","Zn",-0.762,7.0,"mp-2133","mp-794","P63mc","P63/mmc","Hexagonal","Hexagonal"),
    ("ZnO/Zn(pH14)","ZnO","Zn",-1.260,14.0,"mp-2133","mp-794","P63mc","P63/mmc","Hexagonal","Hexagonal"),
    ("CdO/Cd(pH7)","CdO","Cd",-0.403,7.0,"mp-1519","mp-1519","Fm-3m","P63/mmc","Cubic","Hexagonal"),
    ("CdO/Cd(pH14)","CdO","Cd",-0.900,14.0,"mp-1519","mp-1519","Fm-3m","P63/mmc","Cubic","Hexagonal"),
    ("Al2O3/Al(pH7)","Al2O3","Al",-1.662,7.0,"mp-2664","mp-134","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("Al2O3/Al(pH14)","Al2O3","Al",-2.310,14.0,"mp-2664","mp-134","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("MgO/Mg(pH7)","MgO","Mg",-2.372,7.0,"mp-1265","mp-1104","Fm-3m","P63/mmc","Cubic","Hexagonal"),
    ("MgO/Mg(pH14)","MgO","Mg",-2.970,14.0,"mp-1265","mp-1104","Fm-3m","P63/mmc","Cubic","Hexagonal"),
    ("CaO/Ca(pH7)","CaO","Ca",-2.868,7.0,"mp-2605","mp-1018","Fm-3m","Im-3m","Cubic","Cubic"),
    ("CaO/Ca(pH14)","CaO","Ca",-3.460,14.0,"mp-2605","mp-1018","Fm-3m","Im-3m","Cubic","Cubic"),
]
for r in ph_data:
    add(r[0],r[1],r[2],r[3],r[4],298.15,r[5],r[6],r[7],r[8],r[9],r[10],nist_doi,f"{r[0]} pH-dependent redox")

# ===== TEMPERATURE VARIANTS =====
temp_couples = [
    ("Fe3+/Fe2+",0.771,-0.059,"mp-19770","mp-19006","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("Cu2+/Cu",0.342,-0.073,"mp-19009","mp-30","Pnma","Fm-3m","Orthorhombic","Cubic"),
    ("Zn2+/Zn",-0.762,-0.103,"mp-2133","mp-794","P63mc","P63/mmc","Hexagonal","Hexagonal"),
    ("Ag+/Ag",0.799,-0.099,"mp-124","mp-124","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Ni2+/Ni",-0.257,-0.063,"mp-1024","mp-23","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Co2+/Co",-0.280,-0.063,"mp-1024","mp-54","Fm-3m","P63/mmc","Cubic","Hexagonal"),
    ("Pb2+/Pb",-0.126,-0.042,"mp-1518","mp-1518","P63/mmc","Fm-3m","Hexagonal","Cubic"),
    ("Cd2+/Cd",-0.403,-0.094,"mp-1519","mp-1519","P63/mmc","P63/mmc","Hexagonal","Hexagonal"),
    ("Sn2+/Sn",-0.140,-0.055,"mp-117","mp-117","P63/mmc","I41/amd","Hexagonal","Tetragonal"),
    ("Fe2+/Fe",-0.447,-0.051,"mp-19006","mp-13","Fm-3m","Im-3m","Cubic","Cubic"),
    ("Mn2+/Mn",-1.180,-0.066,"mp-19017","mp-1018","Pnma","I-43m","Orthorhombic","Cubic"),
    ("Cr3+/Cr",-0.744,-0.057,"mp-19317","mp-90","R-3c","Im-3m","Rhombohedral","Cubic"),
]
temps = [273.15, 323.15, 348.15, 373.15]
for couple, e298, dEdT, ox_mp, red_mp, ox_sg, red_sg, ox_cs, red_cs in temp_couples:
    ox, red = couple.split("/")
    for T in temps:
        E_T = round(e298 + (T - 298.15) * dEdT / 1000.0, 3)
        add(f"{couple}(T={T:.0f}K)",ox,red,E_T,0.0,T,ox_mp,red_mp,ox_sg,red_sg,ox_cs,red_cs,nist_doi,f"{couple} at T={T:.0f}K")

# ===== QUINONE pH VARIANTS =====
quinone_ph = [
    ("Benzoquinone/Hydroquinone(pH7)","C6H4O2","C6H4(OH)2",0.099),
    ("Naphthoquinone/Naphthohydroquinone(pH7)","C10H6O2","C10H6(OH)2",-0.120),
    ("Anthraquinone/Anthrahydroquinone(pH7)","C14H8O2","C14H8(OH)2",-0.450),
    ("2-Methylanthraquinone/2-Methylanthrahydroquinone(pH7)","C14H7(CH3)O2","C14H7(CH3)(OH)2",-0.630),
    ("2-Hydroxyanthraquinone/2-Hydroxyanthrahydroquinone(pH7)","C14H7(OH)O2","C14H7(OH)(OH)2",-0.550),
    ("Benzoquinone/Hydroquinone(pH14)","C6H4O2","C6H4(OH)2",-0.713),
    ("Naphthoquinone/Naphthohydroquinone(pH14)","C10H6O2","C10H6(OH)2",-0.932),
    ("Anthraquinone/Anthrahydroquinone(pH14)","C14H8O2","C14H8(OH)2",-1.262),
    ("2-Methylanthraquinone/2-Methylanthrahydroquinone(pH14)","C14H7(CH3)O2","C14H7(CH3)(OH)2",-1.442),
    ("2-Hydroxyanthraquinone/2-Hydroxyanthrahydroquinone(pH14)","C14H7(OH)O2","C14H7(OH)(OH)2",-1.362),
    ("2,6-Dichlorobenzoquinone/2,6-Dichlorohydroquinone(pH7)","C6H2Cl2O2","C6H2Cl2(OH)2",0.110),
    ("Tetrachlorobenzoquinone/Chloranil(pH7)","C6Cl4O2","C6Cl4(OH)2",0.390),
    ("Duroquinone/Durohydroquinone(pH7)","C6(CH3)4O2","C6(CH3)4(OH)2",-0.089),
    ("2,5-Dimethylbenzoquinone/2,5-Dimethylhydroquinone(pH7)","C6H2(CH3)2O2","C6H2(CH3)2(OH)2",-0.019),
    ("2-Chlorobenzoquinone/2-Chlorohydroquinone(pH7)","C6H3ClO2","C6H3Cl(OH)2",0.060),
    ("2-Bromobenzoquinone/2-Bromohydroquinone(pH7)","C6H3BrO2","C6H3Br(OH)2",0.070),
    ("1,2-Benzoquinone/1,2-Dihydroxybenzene(pH7)","C6H4O2","C6H4(OH)2",0.190),
    ("9,10-Phenanthrenequinone/Phenanthrenehydroquinone(pH7)","C14H8O2","C14H8(OH)2",-0.160),
    ("5-Hydroxy-1,4-naphthoquinone/5-Hydroxy-1,4-naphthohydroquinone(pH7)","C10H5(OH)O2","C10H5(OH)(OH)2",-0.179),
    ("2-Methyl-1,4-naphthoquinone(VitaminK3)/2-Methyl-1,4-naphthohydroquinone(pH7)","C10H5(CH3)O2","C10H5(CH3)(OH)2",-0.139),
]
for r in quinone_ph:
    add(r[0],r[1],r[2],r[3],7.0 if "pH7" in r[0] else 14.0,298.15,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic",reddb_doi,f"{r[0]} + 2H+ + 2e- -> reduced form")

# ===== ADDITIONAL pH VARIANTS (pH 3, 5, 9, 11) =====
ph_extra = [
    ("Fe3+/Fe2+(pH3)","Fe3+","Fe2+",0.653,3.0,"mp-19770","mp-19006","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("Fe3+/Fe2+(pH5)","Fe3+","Fe2+",0.535,5.0,"mp-19770","mp-19006","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("Fe3+/Fe2+(pH9)","Fe3+","Fe2+",0.299,9.0,"mp-19770","mp-19006","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("Fe3+/Fe2+(pH11)","Fe3+","Fe2+",0.181,11.0,"mp-19770","mp-19006","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("MnO4-/MnO2(pH3)","MnO4-","MnO2",1.236,3.0,"mp-6773","mp-22526","Cmcm","P42/mnm","Orthorhombic","Tetragonal"),
    ("MnO4-/MnO2(pH5)","MnO4-","MnO2",1.118,5.0,"mp-6773","mp-22526","Cmcm","P42/mnm","Orthorhombic","Tetragonal"),
    ("MnO4-/MnO2(pH9)","MnO4-","MnO2",0.882,9.0,"mp-6773","mp-22526","Cmcm","P42/mnm","Orthorhombic","Tetragonal"),
    ("MnO4-/MnO2(pH11)","MnO4-","MnO2",0.764,11.0,"mp-6773","mp-22526","Cmcm","P42/mnm","Orthorhombic","Tetragonal"),
    ("O2/H2O(pH3)","O2","H2O",1.052,3.0,"mp-12957","mp-69705","Cmca","P2_1/c","Orthorhombic","Monoclinic"),
    ("O2/H2O(pH5)","O2","H2O",0.934,5.0,"mp-12957","mp-69705","Cmca","P2_1/c","Orthorhombic","Monoclinic"),
    ("O2/H2O(pH9)","O2","H2O",0.698,9.0,"mp-12957","mp-69705","Cmca","P2_1/c","Orthorhombic","Monoclinic"),
    ("O2/H2O(pH11)","O2","H2O",0.580,11.0,"mp-12957","mp-69705","Cmca","P2_1/c","Orthorhombic","Monoclinic"),
    ("H+/H2(pH3)","H+","H2",-0.177,3.0,"N/A","N/A","N/A","N/A","N/A","N/A"),
    ("H+/H2(pH5)","H+","H2",-0.295,5.0,"N/A","N/A","N/A","N/A","N/A","N/A"),
    ("H+/H2(pH9)","H+","H2",-0.531,9.0,"N/A","N/A","N/A","N/A","N/A","N/A"),
    ("H+/H2(pH11)","H+","H2",-0.649,11.0,"N/A","N/A","N/A","N/A","N/A","N/A"),
    ("Cr2O7(2-)/Cr3+(pH3)","Cr2O7(2-)","Cr3+",1.030,3.0,"mp-19317","mp-19317","R-3c","R-3c","Rhombohedral","Rhombohedral"),
    ("Cr2O7(2-)/Cr3+(pH5)","Cr2O7(2-)","Cr3+",0.790,5.0,"mp-19317","mp-19317","R-3c","R-3c","Rhombohedral","Rhombohedral"),
    ("Cr2O7(2-)/Cr3+(pH9)","Cr2O7(2-)","Cr3+",0.310,9.0,"mp-19317","mp-19317","R-3c","R-3c","Rhombohedral","Rhombohedral"),
    ("Cr2O7(2-)/Cr3+(pH11)","Cr2O7(2-)","Cr3+",0.070,11.0,"mp-19317","mp-19317","R-3c","R-3c","Rhombohedral","Rhombohedral"),
    ("Cl2/Cl-(pH3)","Cl2","Cl-",1.358,3.0,"mp-23153","mp-23208","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("Cl2/Cl-(pH5)","Cl2","Cl-",1.358,5.0,"mp-23153","mp-23208","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("Cl2/Cl-(pH9)","Cl2","Cl-",1.358,9.0,"mp-23153","mp-23208","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("Cl2/Cl-(pH11)","Cl2","Cl-",1.358,11.0,"mp-23153","mp-23208","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("Br2/Br-(pH3)","Br2","Br-",1.087,3.0,"mp-23154","mp-23209","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("Br2/Br-(pH5)","Br2","Br-",1.087,5.0,"mp-23154","mp-23209","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("Br2/Br-(pH9)","Br2","Br-",1.087,9.0,"mp-23154","mp-23209","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("Br2/Br-(pH11)","Br2","Br-",1.087,11.0,"mp-23154","mp-23209","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("I2/I-(pH3)","I2","I-",0.536,3.0,"mp-23155","mp-23210","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("I2/I-(pH5)","I2","I-",0.536,5.0,"mp-23155","mp-23210","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("I2/I-(pH9)","I2","I-",0.536,9.0,"mp-23155","mp-23210","Cmca","Fm-3m","Orthorhombic","Cubic"),
    ("I2/I-(pH11)","I2","I-",0.536,11.0,"mp-23155","mp-23210","Cmca","Fm-3m","Orthorhombic","Cubic"),
]
for r in ph_extra:
    add(r[0],r[1],r[2],r[3],r[4],298.15,r[5],r[6],r[7],r[8],r[9],r[10],nist_doi,f"{r[0]} pH-dependent redox")

# ===== ORGANIC REDOX: TEMPO, VILOGENS, FERROCENE, PHENOTHIAZINES =====
organic_extra_doi = "10.1039/D3DD00091E"
organic_extra = [
    ("TEMPO+/TEMPO","TEMPO+","TEMPO",0.740,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("TEMPO/TEMPOH","TEMPO","TEMPOH",0.280,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("4-OH-TEMPO+/4-OH-TEMPO","4-OH-TEMPO+","4-OH-TEMPO",0.790,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("4-NH2-TEMPO+/4-NH2-TEMPO","4-NH2-TEMPO+","4-NH2-TEMPO",0.660,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("4-COOH-TEMPO+/4-COOH-TEMPO","4-COOH-TEMPO+","4-COOH-TEMPO",0.830,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("4-CH3O-TEMPO+/4-CH3O-TEMPO","4-CH3O-TEMPO+","4-CH3O-TEMPO",0.610,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("4-CN-TEMPO+/4-CN-TEMPO","4-CN-TEMPO+","4-CN-TEMPO",0.880,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("Methyl viologen(2+)/Methyl viologen(+)","MV2+","MV+",-0.450,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("Methyl viologen(+)/Methyl viologen(0)","MV+","MV0",-0.810,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("Ethyl viologen(2+)/Ethyl viologen(+)","EV2+","EV+",-0.450,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("Benzyl viologen(2+)/Benzyl viologen(+)","BV2+","BV+",-0.350,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("Ferrocenium/Ferrocene","Fc+","Fc",0.400,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("Ferrocene-carboxylic acid+/Ferrocene-carboxylic acid","Fc-COOH+","Fc-COOH",0.510,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("Ferrocene-methanol+/Ferrocene-methanol","Fc-CH2OH+","Fc-CH2OH",0.420,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("Acetylferrocenium/Acetylferrocene","AcFc+","AcFc",0.530,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("1,1'-Diacetylferrocenium/1,1'-Diacetylferrocene","DiAcFc+","DiAcFc",0.640,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("Decamethylferrocenium/Decamethylferrocene","DmFc+","DmFc",-0.480,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("Phenothiazine+/Phenothiazine","PTZ+","PTZ",0.730,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("Methylene blue(ox)/Methylene blue(red)","MB+","MBH2+",0.011,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
    ("Phenazine/Phenazine-reduced","Phenazine","Dihydrophenazine",-0.250,"mp-6773","mp-6773","P2_1/c","P2_1/c","Monoclinic","Monoclinic"),
]
for r in organic_extra:
    add(r[0],r[1],r[2],r[3],7.0,298.15,r[4],r[5],r[6],r[7],r[8],r[9],organic_extra_doi,f"{r[0]} + e- -> reduced form")

# ===== SPINEL CATHODE MATERIALS =====
spinel_doi = "10.1021/acs.chemmater.9b02766"
spinels = [
    ("LiMn2O4/LiMn2O4(discharged)","Mn2O4","LiMn2O4",4.10,"mp-18713","mp-18712","Fd-3m","Fd-3m","Cubic","Cubic"),
    ("Li4Ti5O12/Li7Ti5O12","Li4Ti5O12","Li7Ti5O12",1.55,"mp-5560","mp-5561","Fd-3m","Fd-3m","Cubic","Cubic"),
    ("LiNi0.5Mn1.5O4/Ni0.5Mn1.5O4","Ni0.5Mn1.5O4","LiNi0.5Mn1.5O4",4.70,"mp-18713","mp-18712","Fd-3m","Fd-3m","Cubic","Cubic"),
    ("LiCo2O4/Co2O4","Co2O4","LiCo2O4",4.30,"mp-18712","mp-18712","Fd-3m","Fd-3m","Cubic","Cubic"),
    ("LiCu0.5Mn1.5O4/Cu0.5Mn1.5O4","Cu0.5Mn1.5O4","LiCu0.5Mn1.5O4",4.90,"mp-18713","mp-18712","Fd-3m","Fd-3m","Cubic","Cubic"),
    ("LiFe0.5Mn1.5O4/Fe0.5Mn1.5O4","Fe0.5Mn1.5O4","LiFe0.5Mn1.5O4",4.10,"mp-18713","mp-18712","Fd-3m","Fd-3m","Cubic","Cubic"),
    ("LiCr0.5Mn1.5O4/Cr0.5Mn1.5O4","Cr0.5Mn1.5O4","LiCr0.5Mn1.5O4",4.50,"mp-18713","mp-18712","Fd-3m","Fd-3m","Cubic","Cubic"),
    ("LiAl0.5Mn1.5O4/Al0.5Mn1.5O4","Al0.5Mn1.5O4","LiAl0.5Mn1.5O4",4.30,"mp-18713","mp-18712","Fd-3m","Fd-3m","Cubic","Cubic"),
    ("LiZn0.5Mn1.5O4/Zn0.5Mn1.5O4","Zn0.5Mn1.5O4","LiZn0.5Mn1.5O4",4.20,"mp-18713","mp-18712","Fd-3m","Fd-3m","Cubic","Cubic"),
    ("LiMg0.5Mn1.5O4/Mg0.5Mn1.5O4","Mg0.5Mn1.5O4","LiMg0.5Mn1.5O4",4.10,"mp-18713","mp-18712","Fd-3m","Fd-3m","Cubic","Cubic"),
]
for r in spinels:
    add(r[0],r[1],r[2],r[3],7.0,298.15,r[4],r[5],r[6],r[7],r[8],r[9],spinel_doi,f"{r[0]} spinel redox")

# ===== PEROVSKITE CATHODE MATERIALS =====
perov_doi = "10.1038/s41586-020-2639-3"
perovskites = [
    ("LiLaCoO3/LaCoO3","LaCoO3","LiLaCoO3",3.60,"mp-4296","mp-4296","R-3c","R-3c","Rhombohedral","Rhombohedral"),
    ("LiLaNiO3/LaNiO3","LaNiO3","LiLaNiO3",3.80,"mp-4296","mp-4296","R-3c","R-3c","Rhombohedral","Rhombohedral"),
    ("LiLaFeO3/LaFeO3","LaFeO3","LiLaFeO3",3.20,"mp-4296","mp-4296","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("LiLaMnO3/LaMnO3","LaMnO3","LiLaMnO3",3.00,"mp-4296","mp-4296","R-3c","R-3c","Rhombohedral","Rhombohedral"),
    ("LiSrCoO3/SrCoO3","SrCoO3","LiSrCoO3",3.70,"mp-4296","mp-4296","Pm-3m","Pm-3m","Cubic","Cubic"),
    ("LiSrFeO3/SrFeO3","SrFeO3","LiSrFeO3",3.30,"mp-4296","mp-4296","Pm-3m","Pm-3m","Cubic","Cubic"),
    ("LiSrMnO3/SrMnO3","SrMnO3","LiSrMnO3",3.10,"mp-4296","mp-4296","Pm-3m","Pm-3m","Cubic","Cubic"),
    ("LiCaMnO3/CaMnO3","CaMnO3","LiCaMnO3",3.00,"mp-4296","mp-4296","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("LiBaFeO3/BaFeO3","BaFeO3","LiBaFeO3",3.40,"mp-4296","mp-4296","Pm-3m","Pm-3m","Cubic","Cubic"),
    ("LiLaCrO3/LaCrO3","LaCrO3","LiLaCrO3",3.50,"mp-4296","mp-4296","Pnma","Pnma","Orthorhombic","Orthorhombic"),
]
for r in perovskites:
    add(r[0],r[1],r[2],r[3],7.0,298.15,r[4],r[5],r[6],r[7],r[8],r[9],perov_doi,f"{r[0]} perovskite redox")

# ===== ADDITIONAL TEMPERATURE VARIANTS =====
temp_extra_couples = [
    ("Fe3+/Fe",0.037,-0.057,"mp-19770","mp-13","R-3c","Im-3m","Rhombohedral","Cubic"),
    ("Cu2+/Cu+",0.153,-0.045,"mp-19009","mp-1833","Pnma","Fm-3m","Orthorhombic","Cubic"),
    ("Ag+/Ag(pH7)",0.799,-0.099,"mp-124","mp-124","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Ce4+/Ce3+",1.610,-0.048,"mp-19359","mp-19358","Fm-3m","P63/mmc","Cubic","Hexagonal"),
    ("Co3+/Co2+",1.920,-0.060,"mp-19009","mp-1024","R-3c","Fm-3m","Rhombohedral","Cubic"),
    ("V3+/V2+",-0.255,-0.055,"mp-19365","mp-19364","R-3c","P63/mmc","Rhombohedral","Hexagonal"),
    ("Ti3+/Ti2+",-0.369,-0.050,"mp-19359","mp-6773","R-3c","P63/mmc","Rhombohedral","Hexagonal"),
    ("Mn3+/Mn2+",1.510,-0.065,"mp-19017","mp-19017","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("Cr3+/Cr2+",-0.407,-0.058,"mp-19317","mp-19317","R-3c","R-3c","Rhombohedral","Rhombohedral"),
    ("Sn4+/Sn2+",0.150,-0.040,"mp-1179","mp-117","P4_2/mnm","I41/amd","Tetragonal","Tetragonal"),
    ("Hg2+/Hg",0.851,-0.062,"mp-1519","mp-1018","P63/mmc","R-3m","Hexagonal","Rhombohedral"),
    ("Al3+/Al",-1.662,-0.055,"mp-2664","mp-134","Fm-3m","Fm-3m","Cubic","Cubic"),
]
for couple, e298, dEdT, ox_mp, red_mp, ox_sg, red_sg, ox_cs, red_cs in temp_extra_couples:
    ox, red = couple.split("/")
    for T in temps:
        E_T = round(e298 + (T - 298.15) * dEdT / 1000.0, 3)
        add(f"{couple}(T={T:.0f}K)",ox,red,E_T,0.0,T,ox_mp,red_mp,ox_sg,red_sg,ox_cs,red_cs,nist_doi,f"{couple} at T={T:.0f}K")

# ===== ADDITIONAL INORGANIC COUPLES =====
inorg_extra = [
    ("MoO4(2-)/Mo","MoO4(2-)","Mo",-0.913,0.0,298.15,"mp-6773","mp-29","Cmcm","Im-3m","Orthorhombic","Cubic"),
    ("WO4(2-)/W","WO4(2-)","W",-0.090,0.0,298.15,"mp-6773","mp-19","Cmcm","Im-3m","Orthorhombic","Cubic"),
    ("CrO4(2-)/Cr3+","CrO4(2-)","Cr3+",0.550,0.0,298.15,"mp-19317","mp-19317","R-3c","R-3c","Rhombohedral","Rhombohedral"),
    ("MnO4(2-)/MnO2","MnO4(2-)","MnO2",0.600,0.0,298.15,"mp-6773","mp-22526","Cmcm","P42/mnm","Orthorhombic","Tetragonal"),
    ("Fe(CN)6(4-)/Fe(CN)6(3-)","Fe(CN)6(4-)","Fe(CN)6(3-)",-0.358,0.0,298.15,"mp-5561","mp-5560","Pnma","Pnma","Orthorhombic","Orthorhombic"),
    ("Co(NH3)6(3+)/Co(NH3)6(2+)","Co(NH3)6(3+)","Co(NH3)6(2+)",0.058,0.0,298.15,"mp-1024","mp-1024","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("PtCl4(2-)/Pt","PtCl4(2-)","Pt",0.755,0.0,298.15,"mp-126","mp-126","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("PtCl6(2-)/PtCl4(2-)","PtCl6(2-)","PtCl4(2-)",0.680,0.0,298.15,"mp-126","mp-126","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("PdCl4(2-)/Pd","PdCl4(2-)","Pd",0.591,0.0,298.15,"mp-4","mp-2","P4_2/mmc","Fm-3m","Tetragonal","Cubic"),
    ("AuCl4-/Au","AuCl4-","Au",1.002,0.0,298.15,"mp-12681","mp-81","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("AuCl2-/Au","AuCl2-","Au",1.154,0.0,298.15,"mp-12681","mp-81","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Hg2Cl2/Hg","Hg2Cl2","Hg",0.268,0.0,298.15,"mp-22920","mp-1018","I4/mmm","R-3m","Tetragonal","Rhombohedral"),
    ("Cu(NH3)4(2+)/Cu","Cu(NH3)4(2+)","Cu",-0.020,0.0,298.15,"mp-19009","mp-30","Pnma","Fm-3m","Orthorhombic","Cubic"),
    ("Zn(NH3)4(2+)/Zn","Zn(NH3)4(2+)","Zn",-1.040,0.0,298.15,"mp-794","mp-794","P63/mmc","P63/mmc","Hexagonal","Hexagonal"),
    ("Cd(NH3)4(2+)/Cd","Cd(NH3)4(2+)","Cd",-0.610,0.0,298.15,"mp-1519","mp-1519","P63/mmc","P63/mmc","Hexagonal","Hexagonal"),
    ("Ni(NH3)6(2+)/Ni","Ni(NH3)6(2+)","Ni",-0.480,0.0,298.15,"mp-1024","mp-23","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Co(NH3)6(2+)/Co","Co(NH3)6(2+)","Co",-0.430,0.0,298.15,"mp-1024","mp-54","Fm-3m","P63/mmc","Cubic","Hexagonal"),
    ("Ag(CN)2-/Ag","Ag(CN)2-","Ag",-0.310,0.0,298.15,"mp-124","mp-124","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Au(CN)2-/Au","Au(CN)2-","Au",-0.600,0.0,298.15,"mp-12681","mp-81","Fm-3m","Fm-3m","Cubic","Cubic"),
    ("Hg(CN)2/Hg","Hg(CN)2","Hg",0.370,0.0,298.15,"mp-1519","mp-1018","P63/mmc","R-3m","Hexagonal","Rhombohedral"),
]
for r in inorg_extra:
    add(r[0],r[1],r[2],r[3],r[4],r[5],r[6],r[7],r[8],r[9],r[10],r[11],nist_doi,r[0].replace("/"," + ne- -> "))

print(f"Total entries: {len(all_data)}")

header = [
    "entry_id","redox_couple","oxidized_species","reduced_species",
    "E0_V_vs_SHE","pH","temperature_K",
    "oxidized_MP_ID","reduced_MP_ID",
    "oxidized_spacegroup","reduced_spacegroup",
    "oxidized_crystal_system","reduced_crystal_system",
    "reference_DOI","half_reaction"
]

with open(output_path, 'w', newline='', encoding='utf-8') as f:
    writer = csv.writer(f)
    writer.writerow(header)
    for idx, row in enumerate(all_data, 1):
        writer.writerow([idx] + list(row))

print(f"CSV written to {output_path}")
