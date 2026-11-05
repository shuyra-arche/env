use env_initrd::ledger::{
    bundle::{self, Archive, File, Kind, Unit},
    capsule,
};

#[test]
fn capsule_types_are_the_kernel_owner_types() {
    let bytes = boot();
    let capsule: kernel_api_env::ledger::capsule::Capsule<'_> =
        capsule::Capsule::parse(&bytes).unwrap();
    let _: env::ledger::capsule::Region = capsule.region(0).unwrap();
}

fn boot() -> Vec<u8> {
    let mut out = vec![0; capsule::PAGE * 2];
    out[..8].copy_from_slice(&capsule::MAGIC);
    out[8..10].copy_from_slice(&1u16.to_le_bytes());
    out[10] = 12;
    out[12..16].copy_from_slice(&1u32.to_le_bytes());
    out[16..24].copy_from_slice(&0x1000u64.to_le_bytes());
    let length = out.len() as u64;
    out[24..32].copy_from_slice(&length.to_le_bytes());
    for (at, value) in [
        (32, 0x1000u64),
        (40, 1),
        (48, 1),
        (56, capsule::PAGE as u64),
    ] {
        out[at..at + 8].copy_from_slice(&value.to_le_bytes());
    }
    out[64..68].copy_from_slice(&5u32.to_le_bytes());
    out
}

fn sample() -> Vec<u8> {
    bundle::pack(
        &[
            Unit {
                id: "system",
                record: b"unit record",
            },
            Unit {
                id: "pipe",
                record: b"pipe record",
            },
        ],
        &[
            File {
                kind: Kind::Supervisor,
                name: "system-bin",
                bytes: b"elf",
            },
            File {
                kind: Kind::Resource,
                name: "configuration",
                bytes: b"data",
            },
        ],
        0,
        0,
        &boot(),
    )
    .unwrap()
}

#[test]
fn records_and_files_borrow_the_archive_and_keep_independent_identities() {
    let bytes = sample();
    let archive = Archive::parse(&bytes).unwrap();
    assert_eq!(archive.system().id, "system");
    assert_eq!(archive.image().name, "system-bin");
    assert_eq!(archive.units().len(), 2);
    assert_eq!(archive.units().nth(1).unwrap().id, "pipe");
    let resource = archive.find("configuration").unwrap();
    assert_eq!(resource.kind, Kind::Resource);
    assert_eq!(resource.bytes, b"data");
    assert!((bytes.as_ptr_range()).contains(&resource.bytes.as_ptr()));
    let mut files = archive.files();
    assert_eq!(files.by_ref().count(), 2);
    assert!(files.next().is_none());
}

#[test]
fn every_truncation_and_appended_data_is_rejected() {
    let mut bytes = sample();
    for end in 0..bytes.len() {
        assert!(Archive::parse(&bytes[..end]).is_err(), "{end}");
    }
    bytes.push(0);
    assert!(Archive::parse(&bytes).is_err());
}

#[test]
fn corrupted_headers_padding_and_records_are_rejected_without_partial_results() {
    let bytes = sample();
    for at in [0, 4, 8, 12, 14, 16, 20, 24, 28, 32, 36] {
        let mut bad = bytes.clone();
        bad[at..at + 4].fill(255);
        assert!(Archive::parse(&bad).is_err(), "{at}");
    }
    let end = u32::from_le_bytes(bytes[32..36].try_into().unwrap()) as usize;
    let mut bad = bytes.clone();
    bad[end] = 1;
    assert!(Archive::parse(&bad).is_err());
    let mut bad = bytes;
    bad[36..40].copy_from_slice(&0u32.to_le_bytes());
    assert!(Archive::parse(&bad).is_err());
}

#[test]
fn duplicate_names_and_a_resource_boot_image_are_rejected() {
    let units = [Unit {
        id: "same",
        record: b"one",
    }; 2];
    let file = File {
        name: "boot",
        kind: Kind::User,
        bytes: b"elf",
    };
    assert_eq!(
        bundle::pack(&units, &[file], 0, 0, &boot()),
        Err(bundle::Error::Duplicate)
    );
    assert_eq!(
        bundle::pack(&units[..1], &[file, file], 0, 0, &boot()),
        Err(bundle::Error::Duplicate)
    );
    let resource = File {
        kind: Kind::Resource,
        ..file
    };
    assert_eq!(
        bundle::pack(&units[..1], &[resource], 0, 0, &boot()),
        Err(bundle::Error::Record)
    );
}
