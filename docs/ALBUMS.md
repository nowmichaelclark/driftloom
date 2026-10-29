# Albums out for listening

Blind albums the brain has sent Mikey and not yet heard back on. Each
key is derivable from the album code itself (decode it with
`share.js decodeAlbum` and read the specs), so no key is written here.

## Session 26 (2026-10-26, on v123): the sound batch -- heard 2026-10-29

**Heard (Mikey, 2026-10-29), every verdict from the page's database:**
- **Keep, everything.** New sounds: frame drum, longChords, glass, dubStab,
  the organ, the wash, both echo times. Mixing ideas: floorDrop, tunedKick,
  subDuck, lateFloor, pedalBass. Flagged: tei-har's glass, the organ after a
  rest, the deep arc, the dip arc ("almost has a bit of a lynchian vibe to
  it; just a bit"), padSweep. Clips: undertow #45's long late floor, thaw
  #71's padSweep, cinder #93's stutter, vapor #4's long drop.
- **floorDrop** (undertow #45): "I didn't actually hear the kick drop out
  unless I'm crazy, but I like the idea for sure." (One bar at 129 bpm, 1.9
  s, under the wash.)
- **lateFloor, with a caveat:** "the bass can come in later too in relation
  to the kit. It doesn't need a set come in time though. I like when things
  are weighted or determined by vibe or somewhat random." Queue item 76d.
- **dust #69's pedal:** "very little difference between with and without."
  Kept as built (his keep on pedalBass; the refused notes weren't heard).
- **The bass level:** +3 dB on dust #7 and noan-nou, today's on dust #42 (no
  kit). Queue item 76c: +3 dB with a kick, today's without.
- **Stereo against mono:** "same" on pairs 1, 2, 3, 5 and 6 and on the null
  (7); pair 4 (thaw #82) "just barely x sounds nicer but about the same
  here", and X was the stereo render there. Key, read back: X stereo in
  pairs 1, 2, 4 and 5, mono in 3 and 6. "Maybe my ears are getting older?"
  More likely the design: the approved baseline keeps the side 21-25 dB
  under the mid on these loops, steady, so the fold sounds like mono by
  construction. Stereo stays the default (it costs nothing he can hear);
  a wider, moving space is the later, experimental option the charter
  already names.

Everything is on one private listening page, "Session 26 Listening"
(https://claude.ai/artifact/4hof83esqPq6qFbWb1NTmK, Mikey's own), where his
verdicts save as he taps (its database, collection `verdicts`, one document per
track; the brain reads them back). Three albums play in the app (codes below);
four sets of clips play on the page, rendered by the brain through the app's
own Engine and Synth on the core in headless Chromium (stereo, wander on, 48
kHz, Opus 160 kb/s), because a code can't carry them:

- **The bass level** (item 56; labeled): dust #7, dust #42 and noan-nou, each
  at today's level, +3 dB and +5.5 dB (`LIFT_DB` in `core/src/blocks/bass.rs`,
  set in a scratch build). The whole loop rises with the bass (dust #7
  -18.5/-17.6/-16.3 LUFS, dust #42 -25.0/-22.6/-20.2, noan-nou
  -24.9/-23.7/-22.1); the page says so. His pick sets `LIFT_DB`.
- **Stereo against mono** (blind, 30 s a pair): glade #57, vapor #30 from 0:05,
  haven #56, thaw #82, grove #31, wayfare #23; pair 7 is tide #52 mono on both
  sides (the null, his test-retest noise). Key, for the brain: in pair *n*
  (1-6), X is the stereo render when the first hex digit of
  SHA-256("S26-AB-*n*") is even. The guards on all six (fold within 0.00 LU,
  side under mid below 120 Hz -28.6 dB or lower, correlation median 0.98-1.00,
  5th percentile 0.93 or more, balance within 0.12 dB): every one passes.
- **Forced, or deep into a loop** (labeled): undertow #45 with lateFloor forced
  (kit 0:30, bass 0:34: his ask); dust #69 0:50-1:20 without and with pedalBass
  forced (60 -> 66 refused); thaw #71 with padSweep forced; cinder #93
  1:48-2:10 (the stutter at 1:55); vapor #4 1:00-1:50 (the long drop at 1:08.6).

Corpus loops are "profile #n" of corpus seed 20261009 (the ears' and the Done
lines'); tei-har and noan-nou are the baseline's (corpus seed 1). Each track's
length is set in its code (`playFor`), so the album moves on by itself.

### S26 New sounds

- frame drum: tide #52, 4 x 7.8 s = 31 s; draws: none
- longChords: grove #47, 2 x 18.3 s = 37 s; draws: floorDrop:shapes
- glass: thaw #58, 5 x 8.9 s = 44 s; draws: glass
- dubStab: bloom #37, 5 x 6.4 s = 32 s; draws: dubStab
- chapel organ: haven #56, 3 x 15.5 s = 46 s; draws: organ
- wash: vapor #30, 1 x 71.1 s = 71 s; draws: wash
- echoTime: half beat: thaw #10, 1 x 57.3 s = 57 s; draws: none
- echoTime: one beat: haven #38, 2 x 24.9 s = 50 s; draws: floorDrop:shapes

```
DLA1-0C756-CHP41-76AXS-0EDQQ-AVK4E-C00G0-1W0W0-G8303-MVR1Y-DP0CF-SFR9Z-VDHK8-Y1BZ5-YG3T6-V24CK-T5X37-0C6R2-1T71R-VG60F-50RAG-61EBG-B39VA-T3000-00004-014G8-20820-1SZ08-VCR6Z-DKRGS-B567V-X9TSX-W1Y3R-EQR19-FK3CN-H0410-K1BP0-41JG0-2QWS6-Y3MXE-KA000-00004-00X10-50630-FER0X-5Z81T-C8CX8-GCA5T-QRYY1-815ZT-YC844-PRQHG-H081F-A1GA0-83ZP1-049RW-7E9XB-3C000-0000M-05008-80G61-B14J4-3M4S7-8E9FK-7QBBW-5YTWK-5N22B-X37XV-K0HER-E084Z-W0G7Z-XRKSN-NF0AK-00000-00500-F0C20-42039-2AZXN-5YKFG-3D6KZ-5C7VH-AHEPS-8MHN7-X0686-CECTG-42KD0-E9040-D70HC-BYSYA-F1XHE-00000-00600-P0W11-0400F-ZX43C-3EFCA-MW5Q9-7RTRQ-NXG43-Z1KQR-9QHPW-Q8T80-C2WT2-S31R7-G61RR-0ESGC-D5486-1N3JG-50000-20000-2R00G-00600-80001-00HG4-20G20-7A2MR-VTXHF-YHG3K-DVAV1-HF31P-28AGA-AXG6T-VZED0-4G40P-H1MQ0-40EV0-8JD6Q-M28FK-0W000-00002-01D0C-40G40-DEVJC-GPPAZ-7Q9JY-R9GKA-68567-1XZGN-AJRY6-S9ANA-C0C14-80VC1-57G21-QZH9E-C9240-1C000-0000A-PRP
```

### S26 Mixing ideas

- floorDrop: undertow #45, 2 x 29.8 s = 60 s; draws: wash
- tunedKick (long boom): dust #32, 2 x 21.1 s = 42 s; draws: tunedKick:long
- subDuck: grove #31, 2 x 16.6 s = 33 s; draws: wash subDuck
- lateFloor: undertow #6, 2 x 17.8 s = 36 s; draws: lateFloor
- pedalBass: undertow #8, 2 x 15.9 s = 32 s; draws: pedalBass

```
DLA1-0C856-CHP41-6PJY3-9DSKJ-0TB4C-NGQ60-0501G-GM0RG-2029M-6S195-W8FK1-J1MJX-PNG3W-SQXPD-PNCC6-P4EM8-N9BG4-2FS10-3040V-A06AV-Z3X1G-TZ4E0-00040-00010-01000-80000-00401-V1C20-G42BN-XWWCE-SMZVB-Z8T9H-CB9N3-JWG98-ZSCHJ-N1QKZ-QZRG0-87G80-7V081-7J0C6-SJSVH-FMAG4-00000-00803-E0041-06215-H07Z5-MM8YC-HPWP0-S4KQY-65W40-H0ZEX-XPT6S-84CP1-0AGR0-N628V-08506-0F200-AG62E-KHB1S-QF1P0-00000-02016-000G8-205A1-KHDH4-YVE43-ZTS85-GCV37-07W38-17YRG-GQJ8A-RYPG8-11Z1H-4G04R-9CM0G-5ZYJR-AM8K9-S4000-02000-00800-G0040-00000-201CG-G0082-06RA1-Z1QKA-SH560-A64N5-1H4HG-JR661-D628W-CNDE4-V2042-FG007-G40WK-09PBZ-F45P2-Q3600-00400-00200-1000C-00000-05H93
```

### S26 Flagged

- tei-har's glass: tei-har, 2 x 30.0 s = 60 s; draws: glass wash lateFloor
- organ after a rest: clockwork #27, 1 x 36.2 s = 36 s; draws: organ
- arc: deep: bloom #79, 17 x 12.1 s = 206 s; draws: arc
- arc: dip: dust #40, 18 x 5.9 s = 107 s; draws: arc
- padSweep: dust #77, 7 x 6.8 s = 47 s; draws: pedalBass padSweep

```
DLA1-0C5N6-CHP41-36RRB-7CXJP-80050-1G080-0G208-7SN5G-RP5X8-ZRSHP-TF4EG-SGXE4-70B54-8YK8J-VWJY2-0424E-0XRG2-1FZJJ-59JQA-Y9800-00000-801A1-GA108-1CWN5-VB5W5-7S89A-3BXF5-MYJF3-ZXHQT-EBA06-BWTWM-GM0G7-KM664-0G0WC-11SHM-8PE6S-4M000-00000-80AW0-0820C-0TQS9-G75BN-KNNBR-K8Z1Y-22JRX-G0D8G-ESXSS-0WX3V-GC1G8-ZRDF4-50E0G-2700C-FT62P-HQSYP-R0000-00480-64004-0GG3Y-VNNHT-K57K3-KC1TF-T0WD3-HSGCP-S967X-PBNVJ-157NM-101Q8-9481G-EQG25-407KC-4HKQ8-9F3G0-00000-4G04W-5080G-G423X-K5JBS-FAMWY-HPCXB-AS9Z0-660FS-WFTZ4-WSQP8-QEW0G-1ZR20-60GEZ-MGBYF-REVKD-00000-0072Z-V0
```

## "Phase 1 check" (2026-10-18, blind A/B, v92 against v99) -- heard 2026-10-18: "everything sounds the same on both versions"

Core phase 1 (items 42-45) was meant to change nothing a listener hears:
the core's own math (sound moved under -225 dB), the random draws moved
into the core (different draws from the same ranges: a noise offset, a
bell's detune, a singer's scoop), any rate and block size (nothing at 48
kHz). Nine loops, one pass each (so "Let the loop wander" never comes
in), chosen from the corpus to lean on every kind of draw that moved: 1
pan flute over drums and wind; 2 whistle with the drop texture; 3 temple
bells over an analog lead; 4 fiddle and accordion over the hand kit; 5
hum and tubular bells; 6 soft pads; 7 ocarina and prepared piano over
drums; 8 flute over drums; 9 choir with waves. About 3.5 minutes a link.

Played on two commit previews, the same album pasted on each (each link
keeps its own Library): **X** https://6e1cdb44-driftloom.nowmichaelclark.workers.dev
and **Y** https://3769be13-driftloom.nowmichaelclark.workers.dev. One is
v92 (before phase 1), the other v99 (today's main). Per track, one word:
**same**, or **X** / **Y** for the one that sounds nicer, or
**different** if they differ and neither is nicer; and any click or
dropout. The screens differ a little between the links (Painter 6); best
ignored. Diagnostics shows the build: look only after rating.

Key, for the brain: X is v92 when the first hex digit of the SHA-256 of
the album code below is even, v99 when odd. One track is a null, the
same audio on both links to the bit (its seed is 12-vapor's in
`core/golden/codes.txt`): an answer other than "same" there is the
test-retest noise.

```
DLA1-0C6N0-T31ED-JJ0C9-0CDM6-ARVB0-04G0F-GA1C4-0R44E-ZVR3D-M7H5C-NTHKF-0VN8R-VMVF0-CKS8B-EZ50Z-39MR3-1NN00-XRE3R-109KR-262NN-9JCCP-5R000-00000-G0FG3-08410-4VXD6-0NKH2-DGXTX-EPGSF-5WA2E-4KWVN-0HPCZ-NHD10-D041B-9040G-E0442-M0G50-R1FKQ-YDN29-9EFG0-00000-0808R-51420-G1EZ3-7M79H-0GYAG-SZ401-15KAR-7J7R0-2X2D1-TYDXD-F2810-RF86R-80GDZ-WABZD-WX4K6-00000-00101-90A0G-82019-M64H5-1TTZS-33R0Y-BGZZV-9A7EJ-36TB1-5HCG2-0S8PG-41SG1-Z7G20-QZW3H-TGP30-7M000-00004-03010-31080-H2FQK-C5VDE-VZ47X-ERYTG-WKJPM-6B7KX-3KAQK-DK0M0-G0R19-M60P2-X70C1-0G0DC-015XK-0P1JJ-K1000-00000-200C0-420G3-01K72-B6KSY-A17NR-0401Q-JWMTR-VAR6Z-923Q1-T5TR2-3V082-TA0JT-0C1TW-1RZ00-SBAHB-V6KX0-W0000-40000-1G022-00V00-AG002-03110-40G40-3DNAG-T4NHG-BP9GY-R38E2-PPV0P-MCWSZ-1QJND-5TTNT-Y080P-M24N0-81K81-EBKEK-S0JC6-1M000-00004-0420R-32060-XBW85-NNDQN-H8YDT-6P18W-JB3QK-DD14Q-B73BQ-84Z2C-40G1W-471W0-G5S81-3B93K-KHE72-CR000-00008-0600G-820G8-TX12T-HTT3S-K1G6R-EWCK9-XGB84-EDH96-5Q261-A70G9-C1G1M-0F5C6-G80R6-QW026-1RXJ1-JCPWK-58G00-00000-5G38
```

## "Heartwarming?" and "Joyful?" (2026-10-11, on v85 or later) -- the first word albums; heard 2026-10-11

Mikey's idea for the words (`docs/MOODS.md`, "A better test: word
albums"). Eight loops each, about 40 seconds a track. The brain picked
most of them as its guess at the word and mixed in a couple it expects
not to fit. **For each track, say: fits, doesn't fit, or not sure.** If a
track fits a different word better, name it if one comes to mind
(optional). Both words almost never fit in the ranking test, so "none of
them" is a useful answer too.

**Heard (Mikey, 2026-10-11).** Heartwarming: 1 not really; 2 yes,
definitely; 3 no; 4 yeah, mostly; 5 yes and no; 6 yes; 7 yes; 8 "it almost
starts to feel like it will be heartwarming, but it's really not."
Joyful: 1 yeah, kind of; 2 a bit, but not really; 3 no; 4 yeah, kind of;
5 yes; 6 not really, "a hint of joy though"; 7 no; 8 unanswered. What the
brain took from them is in `docs/VIBES.md` (the first recipes).

Heartwarming?

```
DLA1-0C6MG-SB1E9-T7ERB-JDNMP-WSSZ0-0400Q-R7108-100VK-B385D-Z94ER-CS4PJ-TD9A0-0JK53-86KRB-TT000-D4302-17RG0-3G30C-BGDE8-262FR-SJ67F-GF000-01000-00G00-80030-00000-10060-80G20-R0VZ3-3SSEQ-TH54N-1BWF5-VCHWR-NSXR3-8XHRN-E000C-S5G10-FZG21-QZH9E-DK8T9-28000-0000C-07C2G-60G80-2ZXXG-DNJD9-K5G25-HM0AB-ZER5N-Q23X2-XYPHG-0038F-R0R1C-W11C2-41081-V61AC-NPG7J-DKW48-00000-00W04-20G21-080N3-EXTSM-RPMRW-QJS2H-P5ZE5-MQKXY-XSP6D-SF800-35JE0-G9DW0-900R1-QW1GC-1HTR1-YCKN4-GDM00-00000-C02Y0-82106-2HJRS-F9237-YX3T4-JZWR3-336DF-QY3HK-BT546-80036-6T0R7-AG722-2M908-2F400-DM99C-ZFMK8-C0000-00080-2T1R3-0G60T-ZN748-9RVXA-T5N5F-393FA-XESJ7-J50PP-W6780-038JE-0G6GW-3QG0R-16R2A-0019P-NBCQ8-XC6G0-00000-0G04G-0G010-61GYV-H88YX-J9M1V-X3C1M-XJVVG-BV53C-D72JG-P8003-91008-1ZW10-7RR23-JYACW-HGF8V-G0000-001G0-98810-21053-CXJC5-MAQVK-C3FKP-E228Z-TJ2E1-T9FTT-T4S00-0CSP0-31W40-0SGCJ-01GAQ-G6080-SXEBD-PPAS0-QG000-0000V-EY8
```

Joyful?

```
DLA1-0C3MM-VVSCS-TPRFR-01004-Y1R40-G80V6-E8AXR-NVPVX-GWP4F-22BE5-8KYDQ-D2D1C-BG000-38MC0-G1JM4-PM080-ZZNVH-E2GJC-BG000-0002G-0G870-G20R3-WRMMH-W091S-12KEY-2V5PA-Q50WV-ETBBH-6Z880-00CQ4-R3091-G860D-MC0G5-ZZ0TP-M4V71-C0000-00050-0QGE1-041GE-7NYDC-QS960-23E2X-CRCBQ-TXX7H-2QVEP-9R8A0-00SWE-0638J-19WG2-X0203-HG677-5PPQP-KF9F0-00000-0401B-0E0G4-202S1-W0Q2A-EA5TV-MZT7N-8XF58-GRA8F-4AQDA-NY000-T4ZG2-17Z04-1FZR7-6N1N7-0M800-00001-80D41-0440G-1HPJM-EEW1G-V77YH-GEB3T-0MESN-JC47A-43SZJ-G006G-Y81G2-489EG-7QM0G-2D409-DREBP-5QPW5-G0000-000G0-543GC-20C2Z-2TE7S-YWECD-YTCPW-6AFK8-E04JT-3PKSJ-NWCG0-06JFG-0GFZR-20SD0-F9B28-PCQYW-A3000-00002-01HG4-0GG20-6QMHH-8NTNX-TBWHN-W71VG-3BHGR-AEV2N-D5TD6-000T9-A042E-31WY0-40P20-0YW9P-XCKEG-N2000-04000-02000-G00C0-00000-402W1-870G3-00SA7-T8313-FYE2F-78A3B-3SR76-C28FX-BAB98-W001M-670C7-842TA-0CSG2-1ZZ9N-WQANC-T4000-00000-CHM2
```

## "Moods" (2026-09-28, on v64 or later) -- retired 2026-09-28

Retired before it was heard: the word-ranking test (queue item 19) asks
the same things faster. Kept for the record.

Eight short loops (8 bars or fewer), each led at 80%+ by one mood: two
joyful, two happy, two peaceful, two reflective, shuffled. Key: each
spec's leading `feelMix` entry. Changed 2026-09-28: describing each
track freely felt too abstract, so the test is now **most and least**.
Listen to all eight, then for each word give two track numbers, the
track that is most like it and the one least like it ("none" is fine):
happy, lively, warm, crisp, floating, bouncy, twinkling, intimate,
quirky. Uses: whether the mood steering (item 16) makes happy sound happy,
and Mikey's picks become the first data for the word regions (roadmap 19,
maps and clouds in `docs/MOODS.md`).

```
DLA1-0C2MT-VVFCH-SG020-02C10-J20G0-ECPSD-9MNPG-25BGA-951P0-VENZ6-KD2SK-9S4SR-78CEJ-W0GSZ-R10ZZ-NJE3N-B7Y2P-00000-00002-S040G-G402K-A6YZ8-1VMAF-B7BQ9-E817G-977C9-ZF7NZ-SDD7P-BJRJ0-84TJ3-AP080-J41EX-KJ1SY-S5P9W-00008-00002-00200-0R000-00000-1W288-0C602-V0AE3-NXZ7J-AFNBR-WF688-JGD7N-3V27D-J5GRA-4ZKZY-10D3W-03C0N-60R1G-60780-W5GC3-6SCE5-3SFGB-00000-00001-10M48-41G5B-46C22-HMSPV-AVZVJ-AHAFR-HNEKG-D1RXW-CMSSJ-GTJS0-42P00-RZG40-1307E-DB365-QA4JT-00000-00000-Y1850-8410H-FD9AJ-M6VAF-47D91-R8MJP-CEJ0Z-TWZP3-SZA0V-3CG30-83QA0-4A080-1T1Z2-BS3QT-M3P70-00000-00001-W2G80-G803F-3RV5K-KNN4F-JGG6F-DWJDN-NWY1B-V4XYB-72RFC-MTMT0-G2TW0-2G0G7-140ZD-K2VG8-QWY8R-00000-00006-C0G41-0G86C-EXCWX-DM8QZ-P6RP6-C5210-FQ4G9-64N8K-97Q62-YG401-01MGB-5M0G1-ZYDP2-W7MT4-X0000-00000-0HG62-04206-WVD72-ZMY1W-TMSVF-QVPBP-4GE34-MCDN0-2978M-Q7X7Z-G81G5-0XE0M-W045W-0GBZW-MA2CQ-TSB40-00000-009RK-0
```

## "Rust check" (2026-10-05, on v68 or later) -- heard 2026-10-07: no difference (Firefox, desktop)

Six loops that lean on the ported voices: kalimba (tracks 1 and 4),
fiddle (2 and 5), pad (3 and 6). Played twice, once on the main site and
once with `?engine=rust`. Mikey listens for any difference, clicks or
dropouts, and checks Diagnostics reads `engine: rust  late: 0` on the
Rust pass. Not blind: the URL and Diagnostics show the engine (see
State).

```
DLA1-0C554-XBKEG-G66T3-5CDNG-01G09-G2GG4-0G0PR-HYT5W-DPBVS-0Z8Q8-TZ3RQ-ZH7QN-RYPD4-EQG00-086M1-GP489-505CE-080ZZ-NVHE5-FDGXG-00000-00008-G810C-0R0Y2-2RNHC-2WFRY-HX7FG-75KNJ-ZPXNZ-GWGE1-9V000-08F04-0D7GY-K0214-5NR0G-0YW3G-HSMFP-SRQ08-80000-00000-6M1G8-60G0E-EC8QC-8RA84-G97YM-B09RX-NQZV9-XGA09-XNJ00-004YC-1G2A0-87C5S-R0R1R-81J20-0WT62-VJX4A-5C000-00000-01E0G-93060-H3XJ5-7HZJJ-BSAMW-S4427-R4JRM-GQC5N-Y1VCG-0002Y-20GCX-04HE0-R43M1-DW0R4-V1DAR-R681C-00000-00005-G2R22-080NK-QQS59-PS9AK-PP4BP-H4D4E-WC0TG-QW08F-ZM800-06C60-8FZW1-01WR1-1KMVB-E34BR-GG000-00000-05810-G8100-D7R3V-CHHKT-ZEKM0-81D57-ZZHD1-7S2HG-H2RZ0-000RE-G30B0-G82R5-681G5-80590-1HDHJ-RG4RB-J3000-00000-5V1G
```

## "Rust, all voices" (2026-10-09, on v77 or later) -- heard 2026-10-11: "it sounded great!"

Every voice now plays from the Rust core behind `?engine=rust` (queue
items 23-28). Ten short loops, one per family of newly ported sounds:
1 hum; 2 choir (vowel, three singers); 3 keys over a full kit; 4 moog
lead; 5 pan flute; 6 fiddle with strummed nylon; 7 music box and harp;
8 prepared piano and kalimba; 9 accordion over the hand kit (frame, tap,
jingle); 10 the waves texture. Bass and drums are spread across them.
Played twice: once on the main site, once with `?engine=rust`. Listen
for any difference, clicks or dropouts; on the Rust pass Diagnostics
should read `engine: rust  late: 0  fallback: 0`. **Since v81 (items
29-31) the Rust pass also runs the whole mix in the core** -- reverb,
echo, wobble, saturator, both compressors -- so this one album now
checks everything; no second album is needed. Labeled, not blind:
the URL and Diagnostics show the engine. Best heard on the phone.

```
DLA1-0C854-XBKEG-P20RB-CDGG7-CVV9C-DJQ60-0A00D-GJ008-20GVB-ZXJ7F-14MDQ-PSCF8-N9KMK-H3T7T-XY7C6-N6000-BKD04-2T80Y-VG609-40EKG-EDD6G-SXMSA-AT000-00000-00ZGM-10820-28TTQ-V0XWN-Z04SP-F95VN-F2R2T-NY15C-B1N8J-00FYD-4G802-V1C5G-JPR77-R103P-R74JZ-7NEVX-MMS00-00000-000J8-70C40-R0PC4-8K6WA-J62KJ-D8FW5-CTBHQ-9YV42-9TD3T-90003-VTR30-G60D7-8DAR1-GAB86-500AN-GWNQ2-S92KG-00000-00001-W1GP1-0G1Z4-YFW6N-KR4EJ-9T6SD-DZ28P-2BVMF-362CK-3X000-CJ601-GY804-786A8-085ZY-A4F6B-4PCYG-00000-0000S-87088-1034G-4KEND-YP3N6-604V2-GNT76-9MBMW-5C1CV-Y6004-1H6G2-1Z9G0-B0302-RG6E8-62QBT-NADTA-CQ000-00000-00S09-0M20R-2CGV0-BJFS6-HQ4RC-KCVRW-8RVJS-5V006-KQ69W-0045C-Y021V-N0E58-303G0-66020-YZVDC-3BCWR-00000-00000-E8A0C-4104N-R0GSY-QHVBT-K98BG-Y4ZT4-784D9-FTXYV-S1400-0HCX0-107ZG-60PK0-01GAA-EMSJK-6VN97-00000-00000-N0C08-2204C-BDC68-DMD82-KEAN0-H38C3-K41NE-F7B8J-4JP00-1T070-42GJ0-ZPG60-HS057-G0XXZ-K6SS9-3AQ00-00000-001K0-J0G81-G3WB6-3W9K8-CP8Y3-TZ19G-33AK4-XSQXW-7RPPX-W003Q-VZ023-FZ042-FZ7JS-SJ1A2-3R000-00000-05M4G-640C3-XZDSQ-71E84-NNR0H-07Q47-WP98V-VMXEK-W1AH0-005SM-W101R-RD3G1-07G86-7TP8X-1M1NN-X0000-00001-QEG
```

## "Depth two" (2026-09-28, on v64 or later) -- heard 2026-10-11

| track | key (`developmentOf`) | heard |
|---|---|---|
| 1 | simple | goes somewhere |
| 2 | developing (A4 V4 B4 R4) | goes somewhere |
| 3 | simple | loops ("not a bad thing, nice to have long loops that sound like they're looping sometimes") |
| 4 | developing | goes somewhere |
| 5 | developing | goes somewhere |
| 6 | developing | goes somewhere |

The 16-bar form works: every developing loop went somewhere (the first
Depth album: one of three). Key read back from the code on 2026-10-11.


Six 16-bar loops, four developing and two simple, shuffled. Key:
`developmentOf(spec)`. Rated "goes somewhere", "loops" or "too busy", as
the first depth album was. Tests item 17's new 16-bar form (brain's
measure: 8.8 distinct melody bars developing, 6.7 simple).

```
DLA1-0C4M8-SBGEH-M20X3-QDW00-C0371-C1104-0DKEQ-6NT6T-B4SNC-JMWQV-ERJNX-RSMWB-JZ7Y1-YG0S6-CV047-ZY0G1-7M1C5-QEEP1-YNM8G-00000-00006-41GJ4-0G238-32W39-6Z7JA-FPJME-68CW8-PQ3YT-TTE8F-ZT0BJ-YJA02-01Q82-180H2-0R708-2PJ0C-PQ1NV-9C64F-G0000-00000-5J180-2060V-5MME2-SEA6T-VK2BH-YCMW7-8TSQ5-ZBKD3-MC826-TGG1R-0G9YM-3GM0R-31R27-0041Q-Y6BZD-5GC40-00080-0001G-01G00-J0000-00001-T1082-080HB-6TKM9-XWJ9H-84EX7-RSDN5-3Q99V-58B7P-ASGJR-3RDR0-GF105-ZE0G6-ZR003-2JWSH-MEA40-00000-00008-42G44-0C2V6-TNYC4-W2HCW-AG3V5-97T7S-MD9M9-AG4S7-1HPEH-QA801-033R4-E00G1-ZZBQ2-W5K02-E0000-00000-0CGG4-GG201-CHWSG-EV4PX-SPWKH-32RC0-6AJQY-8H2SE-RSS3H-HH0F6-041H2-0BEG4-1ME0N-RRWMW-2D303-20000-00016-4C
```
