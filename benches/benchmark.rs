#[macro_use]
extern crate bencher;
extern crate ticket;

use bencher::Bencher;

fn gen_bench(b: &mut Bencher) {
    let mut t = ticket::Ticket::new();
    b.iter(|| {
        t.gen();
    })
}

fn encode_bench(b: &mut Bencher) {
    let id = ticket::Ticket::new().gen();
    b.iter(|| {
        ticket::encode(id);
    });
}

fn decode_bench(b: &mut Bencher) {
    let id_s = ticket::encode(ticket::Ticket::new().gen());
    b.iter(|| {
        ticket::decode(&id_s);
    });
}

benchmark_group!(benches, gen_bench, encode_bench, decode_bench);
benchmark_main!(benches);
