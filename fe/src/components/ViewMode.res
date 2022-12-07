type note_list_item = {
  id: string,
  title: string,
  category: string,
}

@react.component
let make = (~notes: array<note_list_item>) => {
  <div>
    {notes
    ->Belt.Array.map(note => {
      <div key=note.id>
        <div>
          <strong> {note.title->React.string} </strong>
        </div>
        <div> {note.category->React.string} </div>
      </div>
    })
    ->React.array}
  </div>
}
