import React from 'react'
import ReactDOM from 'react-dom'
import styles from './styles'

class Hello extends React.Component {
  render () {
    return (
      <div style={styles.form}>
        <p>Hello</p>
      </div>
    )
  }
}

ReactDOM.render(
  <Hello />,
  document.getElementById('root')
)
