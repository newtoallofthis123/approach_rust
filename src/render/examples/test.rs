use render::node::Node;

fn main(){
    let node = Node {
        content: Some("First, World! ".to_string()),
        render_id: 0,
        prerender: false,
        nodes: vec![Node {
            content: Some("Hello, World! ".to_string()),
            render_id: 0,
            prerender: false,
            nodes: vec![Node {
                content: Some("Wow, this is a nested node! ".to_string()),
                render_id: 0,
                prerender: false,
                nodes: vec![]
            }, Node {
                content: Some("This is another nested node! ".to_string()),
                render_id: 0,
                prerender: false,
                nodes: vec![]
            }],
        }, Node {
            content: Some("The Matrix has you. ".to_string()),
            render_id: 0,
            prerender: false,
            nodes: vec![]
        }]
    }; 

    println!("{}", node);
}
