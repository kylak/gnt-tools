/* Requires the Docker Pipeline plugin */
pipeline {
    agent { docker { image 'rust:1.87' } }
    stages {
        stage('build') {
            steps {
                sh 'rustc --version'
            }
        }
    }
}
