@module("axios") @val external axiosGet: string => Promise.t<'a> = "get"

%%raw("import './App.css'")

type viewMode = Add | View

type note = {
  id: string,
  title: string,
  body: string,
  category: string,
  created: int,
  last_modified: int,
}

type state = {
  notes: array<ViewMode.note_list_item>,
  username: string,
  view_mode: viewMode,
}

let init_state = {
  notes: [],
  username: "",
  view_mode: View,
}

type actions =
  EnterAddMode | ExitAddMode | UsernameUpdated(string) | NotesLoaded(array<ViewMode.note_list_item>)

let reducer = (state, action) => {
  switch action {
  | EnterAddMode => {
      ...state,
      view_mode: Add,
    }
  | ExitAddMode => {...state, view_mode: View}
  | UsernameUpdated(username) => {...state, username}
  | NotesLoaded(items) => {...state, notes: items}
  }
}

let load_list = (username: string) => {
  axiosGet(ApiConstants.be_url ++ "/users/" ++ username ++ "/notes")
}

@react.component @genType
let make = () => {
  let (state, dispatch) = React.useReducer(reducer, init_state)
  <div className="App">
    <h1> {React.string("Notes App")} </h1>
    <input
      style={ReactDOM.Style.make(~display="block", ~margin="auto", ())}
      value={state.username}
      onChange={e => ReactEvent.Form.currentTarget(e)["value"]->UsernameUpdated->dispatch}
      onBlur={_ => {
        if state.username->Js.String2.length > 0 {
          let _ = load_list(state.username)->Promise.then(data => {
            data["data"]->NotesLoaded->dispatch
            Promise.resolve()
          })
        }
      }}
    />
    <button onClick={_ => dispatch(EnterAddMode)}> {"Add note"->React.string} </button>
    {switch state.view_mode {
    | View => <ViewMode notes=state.notes />
    | Add => <AddMode username=state.username on_exit={() => dispatch(ExitAddMode)} />
    }}
  </div>
}
