pipeline {
  agent { label 'MacMini' }
  parameters {
    gitParameter(name: 'GIT_TAG',
                 type: 'PT_BRANCH_TAG',
                 description: 'The Git tag to checkout. If not specified "master" will be checkout.',
                 defaultValue: 'master')
    booleanParam(name: 'PUBLISH_RESULTS',
                 description: 'Publish the resulting wheels to Pypi.org',
                 defaultValue: false)
  }

  stages {
    stage('Checkout Git TAG') {
      steps {
        cleanWs()
        checkout([$class: 'GitSCM',
                  branches: [[name: "${params.TAG}"]],
                  doGenerateSubmoduleConfigurations: false,
                  extensions: [],
                  gitTool: 'Default',
                  submoduleCfg: [],
                  userRemoteConfigs: [[url: 'https://github.com/eclipse-zenoh/zenoh-python.git']]
                ])
      }
    }

    stage('MacOS wheels') {
      steps {
        sh '''
          for PYTHON_ENV in zenoh-cp35 zenoh-cp36 zenoh-cp37 zenoh-cp38; do
            conda activate ${PYTHON_ENV}
            maturin build --release
          done
        '''
      }
    }

    stage('Manylinux2010-x64 wheels') {
      steps {
        sh '''
          docker run --init -it --rm -v $(pwd):/workdir -w /workdir adlinktech/manylinux2010-x64-rust-nightly maturin build --release --manylinux 2010
        '''
      }
    }

    stage('Manylinux2010-i686 wheels') {
      steps {
        sh '''
          docker run --init -it --rm -v $(pwd):/workdir -w /workdir adlinktech/manylinux2010-i686-rust-nightly maturin build --release --manylinux 2010
        '''
      }
    }

    stage('Deploy to download.eclipse.org') {
      steps {
        sshagent ( ['projects-storage.eclipse.org-bot-ssh']) {
          sh '''
          ssh genie.zenoh@projects-storage.eclipse.org rm -fr /home/data/httpd/download.eclipse.org/zenoh/zenoh-python/${TAG}
          ssh genie.zenoh@projects-storage.eclipse.org mkdir -p /home/data/httpd/download.eclipse.org/zenoh/zenoh-python/${TAG}
          scp target/wheels/*.whl target/wheels/*.tar.gz genie.zenoh@projects-storage.eclipse.org:/home/data/httpd/download.eclipse.org/zenoh/zenoh-python/${TAG}/
          '''
        }
      }
    }

    stage('Deploy on pypi.org') {
      steps {
        sh '''
          if [ "${PUBLISH_RESULTS}" = "true" ]; then
            python3 -m twine upload --repository eclipse-zenoh target/wheels/*.whl target/wheels/*.tar.gz
          else
            echo "Publication to Pypi.org"
          fi
        '''
      }
    }
  }

  post {
    success {
        archiveArtifacts artifacts: 'target/wheels/*.whl, target/wheels/*.tar.gz', fingerprint: true
    }
  }
}