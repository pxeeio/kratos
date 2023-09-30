INSERT INTO muscles (muscle_group, name, simple_name) VALUES
    ('arms', 'Biceps Brachii', 'Biceps'),
    ('arms', 'Triceps Brachii', 'Triceps'),
    ('arms', 'Wrist Extensors', 'Outer Forearm'),
    ('arms', 'Wrist Flexors', 'Inner Forearm'),
    ('back', 'Erector Spinae', 'Lower Back'),
    ('back', 'Latissimus Dorsi', 'Lats'),
    ('back', 'Rhomboids', NULL),
    ('back', 'Lower Trapezius', 'Lower Traps'),
    ('back', 'Middle Trapezius', 'Traps'),
    ('back', 'Upper Trapezius', 'Upper Traps'),
    ('chest', 'Pectoralis Major', 'Pecs'),
    ('core', 'Rectus Abdominis', 'Abs'),
    ('core', 'Transverse Abdominus', 'Transverse Abs'),
    ('core', 'Obliques', NULL),
    ('legs', 'Gastrocnemius', 'Calf'),
    ('legs', 'Gluteus', 'Glutes'),
    ('legs', 'Hamstrings', NULL),
    ('legs', 'Quadriceps', 'Quads'),
    ('shoulders', 'Deltoid', 'Delts'),
    ('shoulders', 'Rotator Cuff', NULL);

INSERT INTO muscles (parent_id, muscle_group, name, simple_name) VALUES
    ((SELECT id FROM muscles WHERE name = 'Pectoralis Major'), 'chest', 'Pectoralis Major, Clavicular Head', 'Upper Pecs'),
    ((SELECT id FROM muscles WHERE name = 'Pectoralis Major'), 'chest', 'Pectoralis Major, Sternal Head', 'Pecs'),
    ((SELECT id FROM muscles WHERE name = 'Deltoid'), 'shoulders', 'Anterior Deltoid', 'Front Delts'),
    ((SELECT id FROM muscles WHERE name = 'Deltoid'), 'shoulders', 'Lateral Deltoid', 'Side Delts'),
    ((SELECT id FROM muscles WHERE name = 'Deltoid'), 'shoulders', 'Posterior Deltoid', 'Rear Delts'),
    ((SELECT id FROM muscles WHERE name = 'Rotator Cuff'), 'shoulders', 'Infraspinatus', NULL),
    ((SELECT id FROM muscles WHERE name = 'Rotator Cuff'), 'shoulders', 'Subscapularis', NULL),
    ((SELECT id FROM muscles WHERE name = 'Rotator Cuff'), 'shoulders', 'Supraspinatus', NULL),
    ((SELECT id FROM muscles WHERE name = 'Rotator Cuff'), 'shoulders', 'Teres Minor', NULL);